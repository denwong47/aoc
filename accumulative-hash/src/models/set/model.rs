//! The actual definition of [`AccumulativeHashSet`], a set model for accumulative hashing.
//!

use crate::{AccumulativeHash, IsAccumulativeHashType};
use nohash::BuildNoHashHasher;
use std::hash::{BuildHasher, Hash};

/// A set model using [`HashSet`] for accumulative hash types.
///
/// This is a convenience wrapper around [`AccumulativeHash`] to track visited,
/// order-independent paths in a DFS-like traversal. It uses a [`HashSet`] with a
/// [`NoHashHasher`] to store and compare visited states efficiently,
/// and uses an internal stack to manage the current path for backtracking.
///
/// [`HashSet`]: std::collections::HashSet
/// [`NoHashHasher`]: nohash::NoHashHasher
pub struct AccumulativeHashSet<T>
where
    T: IsAccumulativeHashType + Hash,
{
    set: std::collections::HashSet<T, nohash::BuildNoHashHasher<T>>,
    hasher: AccumulativeHash<T>,
    path: Vec<T>,
}

impl<T> Default for AccumulativeHashSet<T>
where
    T: IsAccumulativeHashType + Hash,
{
    fn default() -> Self {
        Self {
            set: std::collections::HashSet::default(),
            hasher: AccumulativeHash::new(),
            path: Vec::with_capacity(64),
        }
    }
}

impl<T: IsAccumulativeHashType> AccumulativeHashSet<T>
where
    T: IsAccumulativeHashType + Hash,
    BuildNoHashHasher<T>: BuildHasher,
{
    /// Get a reference to the current hasher state.
    pub fn hasher_state(&self) -> &T {
        self.hasher.state()
    }

    /// Get a count of visited states.
    pub fn visited_count(&self) -> usize {
        self.set.len()
    }

    /// Checks if an element, if added to the current hash state, would lead to a
    /// previously visited state.
    ///
    /// This does not mutate the hash state, only checks for presence.
    pub fn contains_path_to<S: Into<T> + Copy>(&self, element: S) -> bool {
        let final_state = self.hasher.and_hash(element);
        self.set.contains(&final_state)
    }

    /// Advance the hash state by adding an element.
    ///
    /// If the resulting state has been seen before, rollback the hash state and return `false`.
    /// Otherwise return `true`.
    ///
    /// This mutates the hash state, but does not mark the state as visited; this is done
    /// by [`insert`].
    pub fn transverse_to<S: Into<T> + Copy>(&mut self, element: S) -> bool {
        let hash = self.hasher.hash(element);
        let final_state = self.hasher.add_hashed(&hash);

        if self.set.contains(final_state) {
            self.hasher.remove_hashed(&hash);
            false
        } else {
            self.path.push(hash);
            true
        }
    }

    /// Backtrack the last added element, reverting the hash state.
    ///
    /// This does NOT mark the state as visited.
    pub fn backtrack(&mut self) -> bool {
        let last_hash = match self.path.pop() {
            Some(h) => h,
            None => return false,
        };
        self.hasher.remove_hashed(&last_hash);
        true
    }

    /// Mark the current hash state as visited and backtrack the last added element.
    ///
    /// This is a convenience method that combines marking the current state as visited
    /// and backtracking.
    pub fn visit_and_backtrack(&mut self) -> bool {
        let current_state = *self.hasher.state();

        if self.backtrack() {
            self.set.insert(current_state)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulative_hash_set_basic() {
        let mut hash_set = AccumulativeHashSet::<u64>::default();

        assert!(!hash_set.contains_path_to(10u64));
        assert!(hash_set.transverse_to(10u64));
        // 10 -> 10 does not exist
        assert!(!hash_set.contains_path_to(10u64));

        assert!(hash_set.transverse_to(16u64));
        assert!(hash_set.visit_and_backtrack());
        assert!(hash_set.contains_path_to(16u64));

        // It should refuse to transverse to 16 again
        {
            let current_state = *hash_set.hasher.state();
            let current_visited = hash_set.visited_count();
            assert!(!hash_set.transverse_to(16u64));
            assert_eq!(*hash_set.hasher.state(), current_state);
            assert_eq!(hash_set.visited_count(), current_visited);
        }

        // After we backtrack, the state should be now 0
        assert!(hash_set.visit_and_backtrack());
        assert!(hash_set.contains_path_to(10u64));

        assert_eq!(hash_set.visited_count(), 2);
        assert_eq!(*hash_set.hasher.state(), 0u64);

        // Now we go to 16 instead, but `16 -> 10` should already exist
        // because order does not matter
        assert!(hash_set.transverse_to(16u64));
        assert!(hash_set.contains_path_to(10u64));

        // It should refuse to transverse to 10 again
        {
            let current_state = *hash_set.hasher.state();
            let current_visited = hash_set.visited_count();
            assert!(!hash_set.transverse_to(10u64));
            assert_eq!(*hash_set.hasher.state(), current_state);
            assert_eq!(hash_set.visited_count(), current_visited);
        }

        // But it should allow us to go to 20
        assert!(hash_set.transverse_to(20u64));
        assert!(hash_set.visit_and_backtrack());

        assert_eq!(hash_set.visited_count(), 3);
    }
}
