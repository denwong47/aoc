use super::{Container, Requirement, Shape, StateStorage, helpers};
use crate::progress;

use itertools::Itertools;
use kdam::tqdm;

fn set_shape_in_storage(
    bits: &mut StateStorage,
    container: &Container,
    shape: &Shape,
    x: usize,
    y: usize,
) -> anyhow::Result<()> {
    for dy in 0..shape.height() {
        for dx in 0..shape.width() {
            if shape.get(dx, dy) {
                let index = (y + dy) * container.width + (x + dx);
                if index >= container.size() {
                    anyhow::bail!(
                        "Index {index} for ({x}+{dx}, {y}+{dy}) out of bounds for container size {}",
                        container.size()
                    );
                }
                bits.set(index, true);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct PlacementBuilder<'s, 'r, const S: usize> {
    pub shape: &'s Shape,
    pub requirement: &'r Requirement<S>,
    pub x: usize,
    pub y: usize,
    pub shape_count: usize,
}

impl<'s, 'r, const S: usize> PlacementBuilder<'s, 'r, S> {
    pub fn new(shape: &'s Shape, requirement: &'r Requirement<S>, x: usize, y: usize) -> Self {
        Self {
            shape,
            requirement,
            x,
            y,
            shape_count: 0,
        }
    }
}

impl<'s, 'r, const S: usize> Iterator for PlacementBuilder<'s, 'r, S> {
    type Item = Placement<'r, S>;

    fn next(&mut self) -> Option<Self::Item> {
        let instance = Placement::new(
            self.shape,
            self.requirement,
            self.x,
            self.y,
            self.shape_count,
        )
        .unwrap_or_else(|_| {
            panic!(
                "Failed to create placement for shape #{} at ({}, {}) with count {}",
                self.shape.index, self.x, self.y, self.shape_count
            )
        });

        self.shape_count += 1;
        instance
    }
}

#[derive(Debug, Clone)]
pub struct Placement<'r, const S: usize> {
    pub shape_index: usize,
    // This reference is for [`std::fmt::Display`] implementation only
    pub requirement: &'r Requirement<S>,
    pub x: usize,
    pub y: usize,
    pub shape_count: usize,
    state: StateStorage,
}

impl<'r, const S: usize> Placement<'r, S> {
    pub fn new<'s>(
        shape: &'s Shape,
        requirement: &'r Requirement<S>,
        x: usize,
        y: usize,
        shape_count: usize,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(shape_instance_offset) = requirement
            .shape_counts
            .get_shape_instance_offset(shape.index, shape_count)
        {
            let mut bits = requirement.build_new_state_storage();

            // Set the bit for this shape instance
            bits.set(requirement.container.size() + shape_instance_offset, true);

            // Set the bits for the shape's position in the container
            set_shape_in_storage(&mut bits, &requirement.container, shape, x, y)?;

            Ok(Some(Self {
                shape_index: shape.index,
                requirement,
                x,
                y,
                shape_count,
                state: bits,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn state(&self) -> &StateStorage {
        &self.state
    }

    /// Check if the placement fills the container at (x, y)
    pub fn is_filled_at(&self, x: usize, y: usize) -> bool {
        let width = self.requirement.container.width;
        x < width && y < self.requirement.container.height && self.state.get(y * width + x).unwrap_or(false)
    }

    /// Check if the shape instance is set in the placement
    pub fn is_shape_instance_set(&self, shape_index: usize, shape_count: usize) -> bool {
        if let Some(offset) = self
            .requirement
            .shape_counts
            .get_shape_instance_offset(shape_index, shape_count)
        {
            let index = self.requirement.container.size() + offset;
            self.state.get(index).unwrap_or(false)
        } else {
            false
        }
    }
}

impl<'r, const S: usize> std::fmt::Display for Placement<'r, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Placement of the \x1b[36mShape #{}\x1b[0m:\x1b[31m{}\x1b[0m at (\x1b[36m{}\x1b[0m, \x1b[36m{}\x1b[0m):",
            self.shape_index, self.shape_count, self.x, self.y
        )?;

        helpers::display_state_storage::<S>(&self.state, self.requirement, f)
    }
}

/// Pre-compute all possible placements of shapes within the requirement's container.
pub fn build_placements_for_requirement<'r, const S: usize>(
    shapes: &[Shape],
    requirement: &'r Requirement<S>,
) -> Vec<Placement<'r, S>> {
    let total_placements_count = progress::calculate_total_placements(shapes, requirement);
    tqdm!(
        (0..shapes.len())
            .cartesian_product(
                requirement
                    .container
                    .iter_all_positions(shapes[0].width(), shapes[0].height())
            )
            .flat_map(|(shape_index, (x, y))| {
                PlacementBuilder::new(&shapes[shape_index], requirement, x, y)
            }),
        // Set the total count for the progress bar, part of the `tqdm!()` macro
        total = total_placements_count
    )
    .inspect(|placement| {
        assert!(
            !placement.state().is_empty(),
            "Generated placement has empty state!"
        );
    })
    .collect::<Vec<_>>()
}
