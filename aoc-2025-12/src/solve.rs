use itertools::Itertools;

use crate::models::{self, PlacementMask, ShapeCounts, StateStorage, helpers};

#[cfg(feature = "cached-conflicts")]
use kdam::tqdm;

#[cfg(feature = "progress")]
use std::time::Instant;

#[cfg(doc)]
use crate::models::{Container, Requirement};

/// Check if two state storages have any conflicting bits set.
fn is_conflict(state1: &StateStorage, state2: &StateStorage) -> bool {
    !state1.and_cloned(state2).is_empty()
}

/// A private struct to hold the current state during the step-wise search for a fulfillment path.
struct StepStateStore<'r, const S: usize> {
    /// The requirement being fulfilled.
    requirement: &'r models::Requirement<S>,

    /// The maximum placement indices tried at each depth level.
    to_visit: Vec<Vec<usize>>,

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

    /// The available shape counts remaining to be placed.
    available_shape_counts: ShapeCounts<S>,

    /// The remaining shape counts needed to fulfill the requirement.
    required_shape_counts: ShapeCounts<S>,

    /// A set of previously seen states to avoid redundant exploration.
    seen: fxhash::FxHashSet<Vec<usize>>,

    #[cfg(feature = "safeguard")]
    /// A static mask with `1`s at the instance portion to quickly check for solution state.
    ///
    /// A [`StateStorage`] has [`Container::size`] number of bits representing the
    /// ``x by y`` grid of the container, followed by [`Requirement::total_shape_count`] bits
    /// representing the instances of shapes placed.
    ///
    /// This mask has `1`s at the latter portion, allowing quick verification of whether
    /// all instances have been placed.
    instance_state_mask: StateStorage,

    /// Pre-computed cache of conflicts between placements.
    #[cfg(feature = "cached-conflicts")]
    conflicts_cache: Vec<Vec<usize>>,
}

impl<'r, const S: usize> StepStateStore<'r, S> {
    /// Create a new [`StepStateStore`]` for the given requirement and placements length.
    fn new(requirement: &'r models::Requirement<S>, placements: &[models::Placement<S>]) -> Self {
        const FIRST_INDEX: usize = 0;
        
        let current_path = Vec::with_capacity(requirement.total_shape_count());
        
        let available_shape_counts = placements
        .iter()
        .fold(ShapeCounts::new([0usize; S]), |mut counts, placement| {
            counts.increment(placement.shape_index);
            counts
        });

        let placements_len = placements.len();
        let mut instance = Self {
            requirement,
            to_visit: Vec::with_capacity(requirement.total_shape_count()),
            current_path,
            current_state: requirement.build_new_state_storage(),
            deactivated_indices: Vec::with_capacity(placements_len.pow(2)),
            undo_log: Vec::with_capacity(placements_len),
            active_mask: models::build_new_placement_mask(placements_len),
            available_shape_counts,
            required_shape_counts: requirement.shape_counts,
            seen: fxhash::FxHashSet::default(),
            #[cfg(feature = "safeguard")]
            instance_state_mask: requirement.build_instance_state_mask(),
            #[cfg(feature = "cached-conflicts")]
            conflicts_cache: Self::precalculate_conflicts(placements),
        };
        let mut to_visit_first = (FIRST_INDEX+1..placements_len).collect_vec();
        instance.sort_placements_ids_by_shape_demand(&mut to_visit_first, placements);
        instance.to_visit.push(to_visit_first);

        instance.insert_placement_if_compatible(FIRST_INDEX, placements);

        instance
    }

    /// Check if the given placement can be accepted into the current state
    /// without violating any existing placements.
    pub fn can_accept_placement_of(
        &self,
        placement_id: usize,
        _placements: &[models::Placement<S>],
    ) -> bool {
        #[cfg(feature = "cached-conflicts")]
        {
            let known_conflicts = &self.conflicts_cache[placement_id];
            for path_id in &self.current_path {
                // If any placement in the current path conflicts with the new placement,
                // we cannot accept it.
                if known_conflicts.contains(path_id) {
                    return false;
                }
            }
            return true;
        }

        #[cfg(not(feature = "cached-conflicts"))]
        {
            let placement = &_placements[placement_id];
            !is_conflict(&self.current_state, placement.state())
        }
    }
    
    /// Check if there are sufficient available shapes to fulfill the requirement.
    /// 
    /// If any shape type has fewer available shapes than required, return `false`;
    /// otherwise, return `true`.
    pub fn has_sufficient_shapes(&self) -> bool {
        self.available_shape_counts
            .iter()
            .zip(self.required_shape_counts.iter())
            .all(|(available, required)| available >= required)
    }

    /// Only call this function after [`Self::has_sufficient_shapes`] returns `true`.
    fn sort_placements_ids_by_shape_demand(
        &self,
        placement_ids: &mut [usize],
        placements: &[models::Placement<S>],
    ){
        // Calculate shape availability: available - required for each shape type.
        // The lower the availability, the higher the demand, and DFS should prioritize
        // those placements. This allows us to get to `has_sufficient_shapes` failures
        // faster.
        let shape_availability = (0..S).into_iter().fold(
            [0_usize; S],
            |mut acc, shape_index| {
                let required = self
                        .required_shape_counts[shape_index];
                acc[shape_index] = self
                    .available_shape_counts[shape_index]/ required.max(1);
                acc
            }
        );

        placement_ids.sort_by_key(|&idx| {
            shape_availability[placements[idx].shape_index] + idx % 7 // Tie-breaker
        })
    }

    /// Pre-calulate conflicts between placements for faster lookup.
    ///
    /// For the cost of O(n^2) memory and time during initialization,
    /// we can speed up the conflict detection during placement insertion.
    #[cfg(feature = "cached-conflicts")]
    fn precalculate_conflicts(placements: &[models::Placement<S>]) -> Vec<Vec<usize>> {
        let placement_len = placements.len();

        let mut cached_conflicts = vec![Vec::new(); placement_len];
        for i in tqdm!(
            0..placement_len,
            desc = "Pre-calculating placement conflicts"
        ) {
            // Each placement conflicts with itself
            cached_conflicts[i].push(i);
            for j in (i + 1)..placement_len {
                if is_conflict(&placements[i].state(), &placements[j].state()) {
                    cached_conflicts[i].push(j);
                    cached_conflicts[j].push(i);
                }
            }
        }
        cached_conflicts
    }

    /// Register the newly deactivated placements into the store, updating
    /// the active mask, deactivated indices, and undo log accordingly.
    ///
    /// The ``newly_deactivated`` vector contains the indices of placements
    ///
    fn deactivate_placements(&mut self, newly_deactivated: Vec<usize>, placements: &[models::Placement<S>]) {
        #[cfg(feature = "trace")]
        let newly_deactivated_count = newly_deactivated.len();

        self.undo_log.push(self.deactivated_indices.len());

        newly_deactivated.into_iter().for_each(|idx| {
            // TODO check if already deactivated?
            self.active_mask.set(idx, false);
            self.deactivated_indices.push(idx);
            self.available_shape_counts.decrement(placements[idx].shape_index);
        });

        #[cfg(feature = "trace")]
        eprintln!(
            "Deactivated \x1b[31m{}\x1b[0m placements",
            newly_deactivated_count
        );
    }

    /// Undo the last step of placement elimination, restoring the active mask
    /// and eliminated indices to their previous states.
    fn undo_one_step_of_placement_deactivation(&mut self, placements: &[models::Placement<S>]) {
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
                self.available_shape_counts.increment(placements[now_active_idx].shape_index);
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
            .filter(|&idx| !self.can_accept_placement_of(idx, placements))
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

        #[cfg(feature = "safeguard")]
        if !self.can_accept_placement_of(placement_id, placements) {
            return false;
        }

        // Add the placement to the current state
        self.current_state |= placement.state();
        self.current_path.push(placement_id);
        self.required_shape_counts.decrement(placement.shape_index);
        // Don't decrement available_shape_counts here; done in deactivate_placements

        // Mark the current path as seen to avoid redundant exploration
        let mut path_clone = self.current_path.clone();
        path_clone.sort_unstable();
        self.seen.insert(path_clone);

        let newly_eliminated = self.find_incompatible_placements(placements);
        self.deactivate_placements(newly_eliminated, placements);

        // Cache all the available placements for the next depth level
        let mut to_visit = self.iter_available_placements(self.current_path.len()).filter(
            |&idx| {
                let mut potential_path = self.current_path.clone();
                potential_path.push(idx);
                potential_path.sort_unstable();
                
                !self.seen.contains(&potential_path)
            },
        ).collect_vec();
        self.sort_placements_ids_by_shape_demand(&mut to_visit, placements);
        self.to_visit.push(to_visit);

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
        #[cfg(feature = "safeguard")]
        {
            return self.current_state.and_cloned(&self.instance_state_mask)
                == self.instance_state_mask;
        }

        #[cfg(not(feature = "safeguard"))]
        {
            // Since algorithmically we can only reach the full placement count
            // if we have a solution, we can skip the actual check here.
            // This is a performance optimization.
            true
        }
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
    pub fn iter_available_placements(&self, from: usize) -> impl Iterator<Item = usize> + '_ {
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
            self.undo_one_step_of_placement_deactivation(placements);
            self.required_shape_counts.increment(placement.shape_index);
            self.to_visit.pop();


            #[cfg(feature = "trace")]
            eprintln!(
                "Backtracked from placement #{} back to {:?}",
                last_placement_id, self.current_path
            );

            Some(last_placement_id)
        } else {
            #[cfg(feature = "trace")]
            eprintln!("No placements to backtrack from");
            None
        }
    }
}

impl<'r, const S: usize> std::fmt::Display for StepStateStore<'r, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Current \x1b[1mstate storage\x1b[0m:",)?;
        writeln!(f, "Current path: \x1b[36m{:?}\x1b[0m", self.current_path)?;
        writeln!(f, "Available shapes: \x1b[32m{:?}\x1b[0m", self.available_shape_counts)?;
        writeln!(f, "Required shapes : \x1b[33m{:?}\x1b[0m", self.required_shape_counts)?;
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
    let start_of_search = Instant::now();
    #[cfg(feature = "progress")]
    let mut start_of_interval = Instant::now();
    #[cfg(feature = "progress")]
    let mut iter_counter: usize = 0;

    loop {
        #[cfg(feature = "progress")]
        {
            iter_counter += 1;
            if start_of_interval.elapsed().as_secs() >= 1 {
                eprintln!("\x1b[1J\x1b[H{}", step_state);
                eprintln!(
                    "Iterations per second: \x1b[36m{}\x1b[0m",
                    iter_counter / start_of_interval.elapsed().as_secs() as usize
                );
                start_of_interval = Instant::now();
                iter_counter = 0;
            }
        }

        match step_state.current_path.len() {
            // Warning: this `is_solution` check is a no-op unless `safeguard` feature is enabled,
            // since algorithmically we can only reach this depth if we have a solution.
            count if count == total_shape_count && step_state.is_solution() => {
                #[cfg(feature = "trace")]
                eprintln!("{}", step_state);

                #[cfg(feature = "progress")]
                eprintln!(
                    "Search completed in \x1b[36m{:?}\x1b[0m",
                    start_of_search.elapsed()
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
                // Check if we have sufficient shapes remaining to fulfill the requirement.
                if !step_state.has_sufficient_shapes() {
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "\x1b[33mInsufficient shapes remaining to fulfill requirement, backtracking...\x1b[0m"
                    );

                    if step_state.backtrack(placements).is_none() {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "\x1b[33mNo more root placements to try\x1b[0m, search exhausted"
                        );

                        break;
                    }
                    continue;
                }

                // Try to advance the path by finding the next compatible placement.
                // We do not need to try any placements before the last one in the path;
                // Since the results are additive, our paths are always in ascending order of placement IDs.
                let next_placement_id_opt = step_state.to_visit.last_mut()
                    .and_then(|to_visit_at_depth| to_visit_at_depth.pop());

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
                        eprintln!(
                            "\x1b[33mNo more root placements to try\x1b[0m, search exhausted"
                        );

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
                    println!(
                        "{}",
                        helpers::SolutionDisplay::new(
                            &shapes,
                            &placements,
                            fulfillment_path.clone()
                        )
                    );
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

    create_test!(test_example_1(0) = Some(vec![62, 41]));
    create_test!(test_example_2(1) = Some(vec![839, 230, 664, 916, 356, 1067]));
    create_test!(test_example_3(2) = None);
}
