use crate::models::ShapeBuilder;

use super::{Container, ShapeCounts, StateStorage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Requirement<const S: usize> {
    pub container: Container,
    pub shape_counts: ShapeCounts<S>,
}

impl<const S: usize> Requirement<S> {
    pub fn new(container: Container, shape_counts: ShapeCounts<S>) -> Self {
        Self {
            container,
            shape_counts,
        }
    }

    pub fn from_input(line: &str) -> anyhow::Result<Requirement<S>> {
        let (container_part, shapes_part) = line
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("Expected ';' separating container and shapes"))?;

        let (width_str, height_str) = container_part.trim().split_once('x').ok_or_else(|| {
            anyhow::anyhow!("Expected 'x' separating width and height in container part")
        })?;

        let width: usize = width_str
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse container width: {}", e))?;
        let height: usize = height_str
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse container height: {}", e))?;
        let container = Container::new(width, height);

        let shape_counts: [usize; S] = shapes_part
            .split_whitespace()
            .map(|s| {
                s.trim()
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Failed to parse shape count '{}': {}", s, e))
            })
            .enumerate()
            .try_fold([0; S], |mut acc, (index, res)| {
                if index >= S {
                    anyhow::bail!("Expected at most {} shape counts, but found more", S);
                }
                let count = res?;

                acc[index] = count;
                Ok(acc)
            })?;

        let shape_counts = ShapeCounts::new(shape_counts);
        Ok(Requirement::new(container, shape_counts))
    }

    pub fn can_possibly_fit_using(&self, shapes: &[ShapeBuilder]) -> anyhow::Result<bool> {
        let size = self.container.size();

        if shapes.len() != S {
            anyhow::bail!("Expected {} shapes, but found {}", S, shapes.len());
        }

        let total_area = self
            .shape_counts
            .iter()
            .enumerate()
            .fold(0usize, |total_used, (index, count_needed)| {
                total_used + count_needed * shapes[index].count()
            });

        #[cfg(feature = "trace")]
        eprintln!(
            "Total area needed: {}, container size: {}",
            total_area, size
        );

        Ok(total_area <= size)
    }

    pub fn total_shape_count(&self) -> usize {
        self.shape_counts.iter().sum()
    }

    pub fn build_new_state_storage(&self) -> StateStorage {
        StateStorage::zeros(self.container.size() + self.total_shape_count())
    }

    /// Build a mask with `1`s at the instance portion of the state storage.
    pub fn build_instance_state_mask(&self) -> StateStorage {
        let vector_size = self.container.size();
        let mut mask = self.build_new_state_storage();

        for i in vector_size..mask.len() {
            mask.set(i, true);
        }

        mask
    }
}
