use std::fmt::Debug;

use super::Difference;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountArray<T>
where
    T: Clone + Copy + Debug + PartialEq + Eq,
{
    pub(crate) values: Vec<T>,
}

impl<T> From<Vec<T>> for CountArray<T>
where
    T: Clone + Copy + Debug + PartialEq + Eq,
{
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<T> CountArray<T>
where
    T: Debug + Clone + Copy + Default + PartialEq + Eq,
{
    pub fn new(dimension: usize) -> Self {
        Self {
            values: vec![T::default(); dimension],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }
}

impl<T> CountArray<T>
where
    T: Debug + Clone + Copy + Default + PartialEq + Eq + Into<u16>,
{
    // Returns the sum of all values in the CountArray as a u32
    pub fn sum(&self) -> u32 {
        self.iter().fold(0u32, |acc, v| acc + (*v).into() as u32)
    }
}

impl CountArray<u16> {
    pub fn mask(&self) -> CountArray<bool> {
        CountArray {
            values: self.values.iter().map(|v| (v & 1) == 1).collect(),
        }
    }

    pub fn mut_add<T>(&mut self, other: &CountArray<T>) -> anyhow::Result<()>
    where
        T: Into<u16> + Clone + Copy + Debug + PartialEq + Eq,
    {
        self.values
            .iter_mut()
            .zip(other.values.iter())
            .try_for_each(|(mine, yours)| {
                *mine = mine
                    .checked_add((*yours).into())
                    .ok_or_else(|| anyhow::anyhow!("u16 overflowed during addition"))?;

                Ok(())
            })
    }

    pub fn add<T>(&self, other: &CountArray<T>) -> anyhow::Result<Self>
    where
        T: Into<u16> + Clone + Copy + Debug + PartialEq + Eq,
    {
        let mut new = self.clone();
        new.mut_add(other)?;
        Ok(new)
    }

    pub fn difference_from(&self, other: &Self) -> Difference<i16> {
        Difference::from(
            self.iter()
                .zip(other.iter())
                .map(|(mine, yours)| (*mine as i16) - (*yours as i16))
                .collect::<Vec<i16>>(),
        )
    }
}

impl From<&CountArray<u16>> for CountArray<bool> {
    fn from(array: &CountArray<u16>) -> Self {
        array.mask()
    }
}

impl CountArray<bool> {
    pub fn difference_from(&self, other: &Self) -> Difference<bool> {
        Difference::from(
            self.iter()
                .zip(other.iter())
                .map(|(mine, yours)| mine ^ yours)
                .collect::<Vec<bool>>(),
        )
    }

    pub fn display_as_tuple(&self) -> String {
        let elements = self
            .values
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if val { Some(idx.to_string()) } else { None })
            .collect::<Vec<String>>();
        format!("({})", elements.join(", "))
    }
}

#[cfg(test)]
impl PartialEq<&str> for CountArray<bool> {
    fn eq(&self, other: &&str) -> bool {
        let other_values: Vec<bool> = other
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("Could not parse {} into bool", c),
            })
            .collect();

        self.values == other_values
    }
}
