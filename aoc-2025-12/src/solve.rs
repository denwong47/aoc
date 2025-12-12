use itertools::Itertools;

use crate::models::{self, PlacementMask, StateStorage, helpers};

#[cfg(feature="progress")]
use std::time::Instant;

#[cfg(doc)]
use crate::models::{Container, Requirement};

/// A private struct to hold the current state during the step-wise search for a fulfillment path.
struct StepStateStore<'r, const S: usize> {
    /// The requirement being fulfilled.
    requirement: &'r models::Requirement<S>,

    /// The maximum placement indices tried at each depth level.
    max_path: Vec<usize>,

    /// The current path of placement indices being explored.
    current_path: Vec<usize>,

    /// The current state storage representing the fulfillment state.
    current_state: StateStorage,

    /// Indices of placements that have been deactivated so far, in the order they were deactivated.
    ///
    /// Data-wise, this duplicates [`Self::active_mask`], but it allows for efficient undoing of deactivations.
    deactivated_indices: Vec<usize>,

    /// A log to track the lengths of [`Self::deactivated_indices`] at each step,
    /// so that we can undo deactivations when backtracking.
    undo_log: Vec<usize>,

    /// A mask to track which placements are currently active (not deactivated).
    active_mask: PlacementMask,

    /// A static mask with `1`s at the instance portion to quickly check for solution state.
    ///
    /// A [`StateStorage`] has [`Container::size`] number of bits representing the
    /// ``x by y`` grid of the container, followed by [`Requirement::total_shape_count`] bits
    /// representing the instances of shapes placed.
    ///
    /// This mask has `1`s at the latter portion, allowing quick verification of whether
    /// all instances have been placed.
    instance_state_mask: StateStorage,
}

impl<'r, const S: usize> StepStateStore<'r, S> {
    /// Create a new [`StepStateStore`]` for the given requirement and placements length.
    fn new(requirement: &'r models::Requirement<S>, placements: &[models::Placement<S>]) -> Self {
        const FIRST_INDEX: usize = 0;
        let mut max_path = Vec::with_capacity(requirement.total_shape_count());
        max_path.push(FIRST_INDEX);

        let current_path = Vec::with_capacity(requirement.total_shape_count());

        let placements_len = placements.len();
        let mut instance = Self {
            requirement,
            max_path,
            current_path,
            current_state: requirement.build_new_state_storage(),
            deactivated_indices: Vec::with_capacity(placements_len.pow(2)),
            undo_log: Vec::with_capacity(placements_len),
            active_mask: models::build_new_placement_mask(placements_len),
            instance_state_mask: requirement.build_instance_state_mask(),
        };

        instance.insert_placement_if_compatible(FIRST_INDEX, placements);

        instance
    }

    /// Check if the given placement can be accepted into the current state
    /// without violating any existing placements.
    pub fn can_accept_placement_of(&self, placement: &models::Placement<S>) -> bool {
        self.current_state.and_cloned(placement.state()).is_empty()
    }

    /// Register the newly deactivated placements into the store, updating
    /// the active mask, deactivated indices, and undo log accordingly.
    ///
    /// The ``newly_deactivated`` vector contains the indices of placements
    ///
    fn deactivate_placements(&mut self, newly_deactivated: Vec<usize>) {
        #[cfg(feature = "trace")]
        let newly_deactivated_count = newly_deactivated.len();

        self.undo_log.push(self.deactivated_indices.len());

        newly_deactivated.into_iter().for_each(|idx| {
            // TODO check if already deactivated?
            self.active_mask.set(idx, false);
            self.deactivated_indices.push(idx);
        });

        #[cfg(feature = "trace")]
        eprintln!(
            "Deactivated \x1b[31m{}\x1b[0m placements",
            newly_deactivated_count
        );
    }

    /// Undo the last step of placement elimination, restoring the active mask
    /// and eliminated indices to their previous states.
    fn undo_one_step_of_placement_deactivation(&mut self) {
        #[cfg(feature = "trace")]
        let len_before_removal = self.deactivated_indices.len();

        if let Some(last_len) = self.undo_log.pop() {
            while self.deactivated_indices.len() > last_len {
                let now_active_idx = self
                    .deactivated_indices
                    .pop()
                    .expect("Unreachable, eliminated indices should not be empty");

                // Reactivate the placement in the active mask
                self.active_mask.set(now_active_idx, true);
            }
        }
        #[cfg(feature = "trace")]
        eprintln!(
            "Reactivated \x1b[32m{}\x1b[0m out of \x1b[36m{}\x1b[0m placements",
            len_before_removal - self.deactivated_indices.len(),
            self.active_mask.len(),
        );
    }

    /// Look through the currently active placements and find those
    /// that are incompatible with the current state.
    fn find_incompatible_placements(&self, placements: &[models::Placement<S>]) -> Vec<usize> {
        self.active_mask
            .iter_ones()
            .filter(|&idx| !self.can_accept_placement_of(&placements[idx]))
            .collect_vec()
    }

    /// Apply the given placement to the current state,
    /// updating the current state storage.
    ///
    /// Returns ``true`` if the placement was successfully applied,
    /// ``false`` otherwise.
    pub fn insert_placement_if_compatible(
        &mut self,
        placement_id: usize,
        placements: &[models::Placement<S>],
    ) -> bool {
        let placement = &placements[placement_id];

        if !self.can_accept_placement_of(placement) {
            return false;
        }

        // Add the placement to the current state
        self.current_state |= placement.state();
        self.current_path.push(placement_id);

        let newly_eliminated = self.find_incompatible_placements(placements);
        self.deactivate_placements(newly_eliminated);

        // Update the max path, and ensure there is one more entry for the next depth level
        if self.max_path.len() <= self.current_path.len() +1 && self.max_path.len() < self.requirement.total_shape_count() {
            self.max_path.push(self.max_path.len());
        } else {
            self.max_path[self.current_path.len() - 1] = placement_id;
        }

        #[cfg(feature = "trace")]
        eprintln!(
            "Inserted placement #{} into path {:?}, {:?} placements are still active",
            placement_id,
            self.current_path,
            self.active_mask.count_ones()
        );

        true
    }

    /// Check if the current state represents a complete solution,
    /// i.e., all shape instances have been placed.
    pub fn is_solution(&self) -> bool {
        self.current_state.and_cloned(&self.instance_state_mask) == self.instance_state_mask
    }

    /// Take the current solution path if it is a valid solution.
    pub fn take_current_path(self) -> Vec<usize> {
        #[cfg(feature = "trace")]
        {
            if !self.is_solution() {
                eprintln!(
                    "\x1b[33mWarning\x1b[0m: Taking current path which is \x1b[1mnot a solution\x1b[0m"
                );
            }
        }
        self.current_path
    }

    /// Check if there are any available placements left to explore.
    pub fn iter_available_placements(&self) -> impl Iterator<Item = usize> + '_ {
        let from = self.max_path.get(self.current_path.len()).copied().expect(
            "Unreachable; max path should have an entry for the current depth due to `insert_placement_if_compatible`",
        ) + 1;

        // Since the +1 to max_path was preemptive, we may exceed the length of the active mask;
        // Clamp to the length of the active mask.
        self.active_mask.as_bitslice()[from.min(self.active_mask.len())..self.active_mask.len()]
            .iter_ones()
            .map(move |idx| idx + from)
    }

    /// Remove the last placement from the current state,
    /// updating the current state storage and active mask accordingly.
    pub fn backtrack(&mut self, placements: &[models::Placement<S>]) -> Option<usize> {
        if let Some(last_placement_id) = self.current_path.pop() {
            let placement = &placements[last_placement_id];

            // Remove the placement from the current state
            self.current_state ^= placement.state();
            self.undo_one_step_of_placement_deactivation();

            #[cfg(feature = "trace")]
            eprintln!(
                "Backtracked from placement #{} back to {:?}",
                last_placement_id, self.current_path
            );

            // The max path is no longer valid beyond a certain point;
            // Say [i, j] was a dead-end, and we have tried up to [i, j, 0..=n] where n
            // is the max compatible placement at that depth.
            // When we backtrack to [i], we will then try [i, j+1] instead,
            // so the max path at depth 2 (0-based) is no longer valid.
            self.max_path.truncate(self.current_path.len() + 1);
            self.max_path[self.current_path.len()] = last_placement_id + 1;

            Some(last_placement_id)
        } else {
            #[cfg(feature = "trace")]
            eprintln!("No placements to backtrack from");
            return None;
        }
    }
}

impl<'r, const S: usize> std::fmt::Display for StepStateStore<'r, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Current \x1b[1mstate storage\x1b[0m:",
        )?;
        writeln!(
            f,
            "Current path: \x1b[36m{:?}\x1b[0m",
            self.current_path
        )?;
        writeln!(
            f,
            "Max path   : \x1b[31m{:?}\x1b[0m",
            self.max_path
        )?;
        helpers::display_state_storage(&self.current_state, self.requirement, f)
    }
}

/// Using the given pre-computed placements of shapes, perform a Dancing Links to
/// determine if the requirement can be fulfilled.
///
/// This is performed by
/// - DFS through the placements without duplicates, keeping track of the
///   - current path of placements,
///   - a single mutable [`StateStorage`] representing the current fulfillment state
///   - a nested Vec to track eliminated placements at each depth level;
/// - everytime we try to advance the path, we
///   - update the current state storage by applying the placement at the current path's last index
///   - check if the instance count portion of the [`StateStorage`] are all ``1``
pub fn find_one_fulfillment<const S: usize>(
    requirement: &models::Requirement<S>,
    placements: &[models::Placement<S>],
) -> anyhow::Result<Option<Vec<usize>>> {
    #[cfg(feature = "trace")]
    eprintln!(
        "Starting fulfillment search with \x1b[36m{}\x1b[0m placements",
        placements.len()
    );

    let total_shape_count = requirement.total_shape_count();

    let mut step_state = StepStateStore::new(requirement, placements);

    #[cfg(feature = "progress")]
    eprintln!("\x1b[2J"); // Clear screen

    #[cfg(feature = "progress")]
    let mut start_time = Instant::now();
    #[cfg(feature = "progress")]
    let mut iter_counter: usize = 0;

    loop {
        #[cfg(feature = "progress")]
        {
            iter_counter += 1;
            if start_time.elapsed().as_secs() >= 1 {
                eprintln!("\x1b[1J\x1b[H{}", step_state);
                eprintln!(
                    "Iterations per second: \x1b[36m{}\x1b[0m",
                    iter_counter / start_time.elapsed().as_secs() as usize
                );
                start_time = Instant::now();
                iter_counter = 0;
            }
        }

        match step_state.current_path.len() {
            count if count == total_shape_count && step_state.is_solution() => {
                #[cfg(feature = "trace")]
                eprintln!(
                    "\x1b[32mFound solution with path: {:?}\x1b[0m",
                    step_state.current_path
                );

                println!(
                    "{}", step_state
                );

                return Ok(Some(step_state.take_current_path()));
            }
            // This really should be unreachable if the algorithm is correct.
            count if count == total_shape_count => {
                anyhow::bail!(
                    "Unreachable: current path length {} matches total shape count {}, but state is not a solution; how did we insert the last placement?!",
                    count,
                    total_shape_count
                );
            }
            count if count > total_shape_count => {
                anyhow::bail!(
                    "Unreachable: current path length {} exceeds total shape count {}",
                    count,
                    total_shape_count
                );
            }
            count if count < total_shape_count => {
                // Try to advance the path by finding the next compatible placement.
                // We do not need to try any placements before the last one in the path;
                // Since the results are additive, our paths are always in ascending order of placement IDs.
                let next_placement_id_opt = step_state.iter_available_placements().next();

                if let Some(next_placement_id) = next_placement_id_opt {
                    if step_state.insert_placement_if_compatible(next_placement_id, placements) {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Advanced path to \x1b[36m{:?}\x1b[0m",
                            step_state.current_path
                        );
                    } else {
                        eprintln!(
                            "Placement #\x1b[36m{}\x1b[0m is:\n{}",
                            next_placement_id, &placements[next_placement_id]
                        );
                        anyhow::bail!(
                            "Unreachable: next available placement #{} is not compatible",
                            next_placement_id
                        );
                    }
                } else {
                    // No more placements to try at this depth, backtrack.
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "\x1b[33mNo more placements to try\x1b[0m a at current depth, backtracking..."
                    );

                    if step_state.backtrack(placements).is_none() {
                        #[cfg(feature = "trace")]
                        eprintln!("\x1b[33mNo more root placements to try\x1b[0m, search exhausted");

                        break;
                    }
                }
            }
            _ => {
                #[cfg(feature = "trace")]
                eprintln!("\x1b[33mNo more root placements to try\x1b[0m, search exhausted");

                break;
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod test_solve {
    use super::*;
    use crate::_test;

    macro_rules! create_test {
        ($name:ident($requirement:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let (shapes, requirement) = _test::build_all_components($requirement);
                let placements = models::build_placements_for_requirement(&shapes, &requirement);

                let fulfillment_result = find_one_fulfillment(&requirement, &placements)
                    .expect("Failed to find fulfillment");

                if let Some(fulfillment_path) = fulfillment_result.as_ref() {
                    println!("{}", helpers::SolutionDisplay::new(&shapes, &placements, fulfillment_path.clone()));
                }

                let expected: Option<Vec<usize>> = $expected;

                if let Some(expected_path) = expected {
                    let fulfillment_path = fulfillment_result
                        .expect("Expected a fulfillment path, but none was found");
                    assert_eq!(fulfillment_path, expected_path);
                } else {
                    assert!(
                        fulfillment_result.is_none(),
                        "Expected no fulfillment path, but one was found: {:?}",
                        fulfillment_result.unwrap()
                    );
                }
            }
        };
    }

    create_test!(test_example_1(0) = Some(vec![2, 21]));
    create_test!(test_example_2(1) = Some(vec![50, 368, 300, 507, 854, 879]));
    create_test!(test_example_3(2) = None);
}
