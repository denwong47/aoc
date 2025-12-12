use crate::models::Placement;

use super::{EMPTY_DISPLAY, FILLED_DISPLAY, Requirement, Shape, StateStorage};

pub fn display_state_storage<const S: usize>(
    state: &StateStorage,
    requirement: &Requirement<S>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for row in 0..requirement.container.height {
        write!(f, "{:3}: ", row)?;
        for col in 0..requirement.container.width {
            let index = row * requirement.container.width + col;
            let ch = if state
                .get(index)
                .expect("Index out of bounds in bits vector")
            {
                FILLED_DISPLAY
            } else {
                EMPTY_DISPLAY
            };
            write!(f, "{}", ch)?;
        }
        writeln!(f)?;
    }

    writeln!(f, "Shape instance:")?;
    write!(f, "{:5}", "")?;
    for idx in 0..requirement.shape_counts.max() {
        write!(f, "{}", idx % 10)?;
    }
    writeln!(f)?;
    for shape_index in 0..S {
        write!(f, "{:>3}: ", shape_index)?;

        for shape_count in 0..requirement.shape_counts[shape_index] {
            let instance_index = requirement.container.size()
                + requirement
                    .shape_counts
                    .get_shape_instance_offset(shape_index, shape_count)
                    .expect("Shape instance offset should exist");
            let ch = if state
                .get(instance_index)
                .expect("Index out of bounds in bits vector")
            {
                FILLED_DISPLAY
            } else {
                EMPTY_DISPLAY
            };
            write!(f, "{}", ch)?;
        }

        writeln!(f)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct SolutionDisplay<'r, 's, 'p, const S: usize> {
    pub shapes: &'s [Shape],
    pub placements: &'r [Placement<'p, S>],
    pub solution: Vec<usize>,
}

impl<'r, 's, 'p, const S: usize> SolutionDisplay<'r, 's, 'p, S> {
    pub fn new(
        shapes: &'s [Shape],
        placements: &'r [Placement<'p, S>],
        solution: Vec<usize>,
    ) -> Self {
        Self {
            shapes,
            placements,
            solution,
        }
    }
}

impl<'r, 's, 'p, const S: usize> std::fmt::Display for SolutionDisplay<'r, 's, 'p, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.placements.is_empty() || self.solution.is_empty() {
            writeln!(f, "No placements or solutions available")?;
            return Ok(());
        }
        let requirement = &self.placements[0].requirement;
        let relevant_placements: Vec<&Placement<S>> = self
            .solution
            .iter()
            .filter_map(|&index| self.placements.get(index))
            .collect();

        writeln!(f, "\x1b[34m\x1b[1mSolution found:\x1b[0m")?;
        writeln!(
            f,
            "To fill the container of size \x1b[36m{}\x1b[0mx\x1b[36m{}\x1b[0m with the \x1b[36m{}\x1b[0m specified shapes:",
            requirement.container.width,
            requirement.container.height,
            requirement.total_shape_count()
        )?;
        writeln!(f)?;
        for row in 0..requirement.container.height {
            write!(f, "{:3}: ", row)?;
            for col in 0..requirement.container.width {
                write!(
                    f,
                    "{}",
                    relevant_placements
                        .iter()
                        .find_map(|placement| {
                            placement
                                .is_filled_at(col, row)
                                .then(|| self.shapes[placement.shape_index].display_filled())
                        })
                        .unwrap_or_else(|| EMPTY_DISPLAY.to_string())
                )?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Shape instance:")?;
        write!(f, "{:5}", "")?;
        for idx in 0..requirement.shape_counts.max() {
            write!(f, "{}", idx % 10)?;
        }
        writeln!(f)?;
        for shape_index in 0..S {
            write!(f, "{:>3}: ", shape_index)?;

            for shape_count in 0..requirement.shape_counts[shape_index] {
                write!(
                    f,
                    "{}",
                    relevant_placements
                        .iter()
                        .find_map(|placement| {
                            placement
                                .is_shape_instance_set(shape_index, shape_count)
                                .then(|| self.shapes[placement.shape_index].display_filled())
                        })
                        .unwrap_or_else(|| EMPTY_DISPLAY.to_string())
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
