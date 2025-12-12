use crate::{
    models::{self, container},
    progress,
};
use bitvec_simd::BitVec;

use itertools::Itertools;
use kdam::tqdm;

pub type MaskStorage = BitVec;

fn set_shape_in_storage(
    bits: &mut MaskStorage,
    container: &container::Container,
    shape: &models::Shape,
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
    pub shape: &'s models::Shape,
    pub requirement: &'r models::Requirement<S>,
    pub x: usize,
    pub y: usize,
    pub shape_count: usize,
}

impl<'s, 'r, const S: usize> PlacementBuilder<'s, 'r, S> {
    pub fn new(
        shape: &'s models::Shape,
        requirement: &'r models::Requirement<S>,
        x: usize,
        y: usize,
    ) -> Self {
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
        .unwrap_or_else(|_| panic!("Failed to create placement for shape #{} at ({}, {}) with count {}",
            self.shape.index, self.x, self.y, self.shape_count));

        self.shape_count += 1;
        instance
    }
}

#[derive(Debug, Clone)]
pub struct Placement<'r, const S: usize> {
    pub shape_index: usize,
    pub requirement: &'r models::Requirement<S>,
    pub x: usize,
    pub y: usize,
    pub shape_count: usize,
    bits: MaskStorage,
}

impl<'r, const S: usize> Placement<'r, S> {
    pub fn new<'s>(
        shape: &'s models::Shape,
        requirement: &'r models::Requirement<S>,
        x: usize,
        y: usize,
        shape_count: usize,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(shape_instance_offset) = requirement
            .shape_counts
            .get_shape_instance_offset(shape.index, shape_count)
        {
            let vector_size = requirement.container.size() + requirement.total_shape_count();
            let mut bits = BitVec::zeros(vector_size);

            bits.set(requirement.container.size() + shape_instance_offset, true);

            set_shape_in_storage(&mut bits, &requirement.container, shape, x, y)?;

            Ok(Some(Self {
                shape_index: shape.index,
                requirement,
                x,
                y,
                shape_count,
                bits,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn bits(&self) -> &BitVec {
        &self.bits
    }
}

impl<'r, const S: usize> std::fmt::Display for Placement<'r, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Placement of the \x1b[36mShape #{}\x1b[0m:\x1b[31m{}\x1b[0m at (\x1b[36m{}\x1b[0m, \x1b[36m{}\x1b[0m):",
            self.shape_index, self.shape_count, self.x, self.y
        )?;
        for row in 0..self.requirement.container.height {
            write!(f, "{:3}: ", row)?;
            for col in 0..self.requirement.container.width {
                let index = row * self.requirement.container.width + col;
                let ch = if self
                    .bits
                    .get(index)
                    .expect("Index out of bounds in bits vector")
                {
                    models::shape::FILLED_DISPLAY
                } else {
                    models::shape::EMPTY_DISPLAY
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Shape instance:")?;
        write!(f, "{:5}", "")?;
        for idx in 0..self.requirement.shape_counts.max() {
            write!(f, "{}", idx % 10)?;
        }
        writeln!(f)?;
        for shape_index in 0..S {
            write!(f, "{:>3}: ", shape_index)?;

            for shape_count in 0..self.requirement.shape_counts[shape_index] {
                let instance_index = self.requirement.container.size()
                    + self
                        .requirement
                        .shape_counts
                        .get_shape_instance_offset(shape_index, shape_count)
                        .expect("Shape instance offset should exist");
                let ch = if self
                    .bits
                    .get(instance_index)
                    .expect("Index out of bounds in bits vector")
                {
                    models::shape::FILLED_DISPLAY
                } else {
                    models::shape::EMPTY_DISPLAY
                };
                write!(f, "{}", ch)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn build_placements_for_requirement<'r, const S: usize>(
    shapes: &[models::Shape],
    requirement: &'r models::Requirement<S>,
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
                models::PlacementBuilder::new(&shapes[shape_index], requirement, x, y)
            }),
        total = total_placements_count
    )
    .collect::<Vec<_>>()
}
