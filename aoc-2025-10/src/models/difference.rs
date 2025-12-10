#[derive(Debug, Clone)]
pub enum Difference<T> {
    /// All values are exactly equal, this is a solution. Not necessarily optimal,
    /// but a solution nonetheless.
    Equal,
    /// Some values are smaller than they need to be.
    /// This means that more additions are required to reach the target.
    Incomplete(Vec<T>),
    /// Some values are bigger than they need to be.
    /// Since we can't add negative numbers, this indicates a no-solution path.
    Overshot(Vec<T>),
}

impl<T> From<Vec<T>> for Difference<T>
where
    T: Eq + PartialOrd + Default + std::fmt::Debug,
{
    fn from(values: Vec<T>) -> Self {
        let (is_all_zeros, has_negative) =
            values
                .iter()
                .fold((true, false), |(all_zeros, neg_found), v| {
                    (
                        all_zeros && *v == T::default(),
                        neg_found || *v < T::default(),
                    )
                });

        match (is_all_zeros, has_negative) {
            (true, _) => Difference::Equal,
            (false, false) => Difference::Incomplete(values),
            (false, true) => Difference::Overshot(values),
        }
    }
}

impl Difference<i16> {
    pub fn distance(&self) -> u64 {
        match self {
            Difference::Equal => 0,
            Difference::Incomplete(vals) => vals.iter().map(|v| (*v as u64).pow(2)).sum(),
            Difference::Overshot(..) => u64::MAX,
        }
    }
}

impl std::cmp::PartialEq for Difference<i16> {
    fn eq(&self, other: &Self) -> bool {
        self.distance() == other.distance()
    }
}

impl std::cmp::Eq for Difference<i16> {}

impl std::cmp::PartialOrd for Difference<i16> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Difference<i16> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance().cmp(&other.distance())
    }
}

pub struct PartialOrdered<U> {
    pub difference: Difference<i16>,
    pub data: U,
}

impl<U> std::cmp::PartialEq for PartialOrdered<U> {
    fn eq(&self, other: &Self) -> bool {
        self.difference == other.difference
    }
}

impl<U> std::cmp::Eq for PartialOrdered<U> {}

impl<U> std::cmp::PartialOrd for PartialOrdered<U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.difference.partial_cmp(&other.difference)
    }
}

impl<U> std::cmp::Ord for PartialOrdered<U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.difference.cmp(&other.difference)
    }
}
