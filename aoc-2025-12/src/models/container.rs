#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Container {
    pub width: usize,
    pub height: usize,
}

impl Container {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn iter_all_positions(
        &self,
        shape_width: usize,
        shape_height: usize,
    ) -> impl Iterator<Item = (usize, usize)> + Clone {
        let max_x = self.width.saturating_sub(shape_width);
        let max_y = self.height.saturating_sub(shape_height);
        (0..=max_y).flat_map(move |y| (0..=max_x).map(move |x| (x, y)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCounts<const S: usize>([usize; S]);

impl<const S: usize> ShapeCounts<S> {
    pub fn new(counts: [usize; S]) -> Self {
        Self(counts)
    }

    pub fn max(&self) -> usize {
        *self.0.iter().max().unwrap_or(&0)
    }

    pub fn get_shape_instance_offset(
        &self,
        shape_index: usize,
        shape_count: usize,
    ) -> Option<usize> {
        let count_for_this_shape = *self.0.get(shape_index)?;
        if count_for_this_shape <= shape_count {
            return None;
        }
        Some(self.0.iter().take(shape_index).sum::<usize>() + shape_count)
    }
}

impl<const S: usize> std::ops::Index<usize> for ShapeCounts<S> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const S: usize> std::ops::Deref for ShapeCounts<S> {
    type Target = [usize; S];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod shape_count_offset {
        use super::*;
        macro_rules! create_test {
            ($name:ident($shape_counts:expr).get_shape_instance_offset($shape_index:expr, $shape_count:expr) = $expected:expr) => {
                #[test]
                fn $name() {
                    let shape_counts = ShapeCounts::new($shape_counts);
                    let result = shape_counts.get_shape_instance_offset($shape_index, $shape_count);
                    assert_eq!(result, $expected);
                }
            };
        }

        create_test!(test1([2, 3, 1]).get_shape_instance_offset(0, 0) = Some(0));
        create_test!(test2([2, 3, 1]).get_shape_instance_offset(0, 1) = Some(1));
        create_test!(test3([2, 3, 1]).get_shape_instance_offset(1, 0) = Some(2));
        create_test!(test4([2, 3, 1]).get_shape_instance_offset(1, 2) = Some(4));
        create_test!(test5([2, 3, 1]).get_shape_instance_offset(2, 0) = Some(5));
        create_test!(test6([2, 3, 1]).get_shape_instance_offset(2, 1) = None);
        create_test!(test7([2, 3, 1]).get_shape_instance_offset(0, 3) = None);
    }
}
