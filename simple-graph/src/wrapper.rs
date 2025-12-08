use std::ops::Deref;

/// A wrapper struct that indicates the contained item does not have a specific order,
/// but the wrapper itself will always be [`Eq`] and [`Ord`], returning [`std::cmp::Ordering::Equal`]
/// for all comparisons.
pub struct UnorderedItem<T>(pub T);

impl<T> UnorderedItem<T> {
    /// Create a new [`UnorderedItem`] wrapping the given item.
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Consume the wrapper and return the inner item.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for UnorderedItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> PartialEq for UnorderedItem<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for UnorderedItem<T> {}

impl<T> PartialOrd for UnorderedItem<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<T> Ord for UnorderedItem<T> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}
