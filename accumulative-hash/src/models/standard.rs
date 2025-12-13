//! The standard model for accumulative hashing, without atomic types.

use crate::{IsAccumulativeHashType, helpers};

/// A struct that remembers the state of a hash as data is added and/or removed from it.
///
/// The order of the item is NOT considered when calculating the hash --
/// ``A-B-C`` will have the same hash as ``C-B-A``. This is deliberate, and useful for
/// addictive data structures that need to check for equality regardless of order,
/// where traditional hashing requires sorting beforehand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccumulativeHash<T: IsAccumulativeHashType> {
    state: T,
}

impl<T: IsAccumulativeHashType> AccumulativeHash<T> {
    /// Create a new empty accumulative hash.
    ///
    /// The initial state is equivalent to hashing no values.
    pub fn new() -> Self {
        Self::with_state(T::zero())
    }

    /// Create a new accumulative hash with an initial state.
    pub fn with_state(state: T) -> Self {
        Self { state }
    }

    /// Add a value to the accumulative hash.
    ///
    /// This does not guarantee that the value was never added before;
    /// it will simply add the hashed value to the current state.
    ///
    /// This means that adding the same value multiple times will
    /// affect the hash state accordingly.
    pub fn add<S: Into<T>>(&mut self, value: S) -> &T {
        let hashed = helpers::hash::<T, _>(value.into());
        self.state = self.state.wrapping_add(&hashed);

        self.state()
    }

    /// Remove a value from the accumulative hash.
    ///
    /// This does not guarantee that the value was previously added;
    /// it will simply subtract the hashed value from the current state.
    ///
    /// This means that removing a value that was never added may lead to
    /// undetermined behavior; it can be fixed by re-adding the value later,
    /// but the intermediate state may not be valid.
    pub fn remove<S: Into<T>>(&mut self, value: S) -> &T {
        let hashed = helpers::hash::<T, _>(value.into());
        self.state = self.state.wrapping_sub(&hashed);

        self.state()
    }

    /// Add multiple values to the accumulative hash.
    pub fn add_multiple<S: Into<T>, I: IntoIterator<Item = S>>(&mut self, values: I) -> &T {
        for value in values {
            self.add(value);
        }
        self.state()
    }

    /// Remove multiple values from the accumulative hash.
    pub fn remove_multiple<S: Into<T>, I: IntoIterator<Item = S>>(&mut self, values: I) -> &T {
        for value in values {
            self.remove(value);
        }
        self.state()
    }

    /// Get the current state of the accumulative hash.
    pub fn state(&self) -> &T {
        &self.state
    }

    /// Extend this accumulative hash by merging another accumulative hash into it.
    ///
    /// Hashing in this way guarantees: ``hash([A]) + hash([B]) == hash([A, B])`` where
    /// ``hash`` represents [`AccumulativeHash::add_multiple`] on different instances of
    /// [`AccumulativeHash`].
    ///
    /// This allows us to combine two accumulative hashes into one, effectively merging
    /// their states.
    ///
    /// Since ``T`` implements [`Copy`], we can afford to copy the state of the other
    /// accumulative hash without worrying about cost.
    pub fn extend(&mut self, other: &AccumulativeHash<T>) -> &T {
        self.state = self.state.wrapping_add(&other.state);
        &self.state
    }

    /// Consume this accumulative hash and return its current state.
    pub fn into_state(self) -> T {
        self.state
    }
}

/// [`AccumulativeHash`] can be created from any iterable collection of values.
impl<T: IsAccumulativeHashType, I> From<I> for AccumulativeHash<T>
where
    I: IntoIterator,
    I::Item: Into<T>,
{
    /// Create an accumulative hash from an iterable collection of values.
    fn from(value: I) -> Self {
        let mut acc_hash = AccumulativeHash::<T>::new();
        acc_hash.add_multiple(value);
        acc_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEQUENCE_TO_ADD_1: &'static [u8] = &[1, 2, 4, 8, 16, 32, 64, 128];
    const SEQUENCE_TO_REMOVE_1: &'static [u8] = &[1, 4, 8, 64];
    const SEQUENCE_TO_ADD_2: &'static [u8] = &[3, 6, 9, 12, 15];
    const SEQUENCE_TO_REMOVE_2: &'static [u8] = &[2, 6, 12];

    macro_rules! test_type {
        ($name:ident::<$typ:ident>(add_1=$add_1:literal, remove_1=$remove_1:literal, add_2=$add_2:literal, remove_2=$remove_2:literal)) => {
            mod $name {
                use super::*;
                use num_traits::Zero;
                use std::collections::HashSet;

                #[test]
                fn sequential_add_must_equal_multiple_add() {
                    let mut acc_hash_seq = AccumulativeHash::<$typ>::new();
                    for &value in SEQUENCE_TO_ADD_1.iter() {
                        acc_hash_seq.add(value);
                    }
                    let state_seq = *acc_hash_seq.state();

                    let mut acc_hash_multi = AccumulativeHash::<$typ>::new();
                    acc_hash_multi.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    let state_multi = *acc_hash_multi.state();

                    assert_eq!(state_seq, state_multi, "Sequential add and multiple add states do not match.");
                }

                #[test]
                fn sequential_add_must_equal_to_unordered_add() {
                    let mut acc_hash_seq = AccumulativeHash::<$typ>::new();
                    let state_seq = *acc_hash_seq.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());

                    let mut acc_hash_unordered = AccumulativeHash::<$typ>::new();
                    acc_hash_unordered.add_multiple(SEQUENCE_TO_ADD_1.iter().rev().cloned());
                    let state_unordered = *acc_hash_unordered.state();

                    assert_eq!(state_seq, state_unordered, "Sequential add and unordered add states do not match.");
                }

                #[test]
                fn sequential_remove_must_equal_multiple_remove() {
                    let mut acc_hash_seq = AccumulativeHash::<$typ>::new();
                    acc_hash_seq.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());

                    let mut acc_hash_unordered = acc_hash_seq.clone();

                    let state_seq = *acc_hash_seq.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned());
                    let state_multi = *acc_hash_unordered.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().rev().cloned());

                    assert_eq!(state_seq, state_multi, "Sequential remove and multiple remove states do not match.");
                }

                #[test]
                fn sequential_add_and_remove() {
                    let mut acc_hash = AccumulativeHash::<$typ>::new();

                    let expected_after_add_1 = $add_1 as $typ;
                    let actual_after_add_1 = *acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    assert!(actual_after_add_1 == expected_after_add_1, "Hashes not matching after first addition: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_add_1, stringify!(==), expected_after_add_1);

                    let expected_after_remove_1 = $remove_1 as $typ;
                    let actual_after_remove_1 = *acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned());
                    assert!(actual_after_remove_1 == expected_after_remove_1, "Hashes not matching after first removal: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_remove_1, stringify!(==), expected_after_remove_1);
                    let expected_after_add_2 = $add_2 as $typ;
                    let actual_after_add_2 = *acc_hash.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned());
                    assert!(actual_after_add_2 == expected_after_add_2, "Hashes not matching after second addition: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_add_2, stringify!(==), expected_after_add_2);

                    let expected_after_remove_2 = $remove_2 as $typ;
                    let actual_after_remove_2 = *acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned());
                    assert!(actual_after_remove_2 == expected_after_remove_2, "Hashes not matching after second removal: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_remove_2, stringify!(==), expected_after_remove_2);
                }

                #[test]
                fn adding_and_removing_same_values_must_return_to_initial_state() {
                    let mut acc_hash = AccumulativeHash::<$typ>::new();

                    acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    acc_hash.remove_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    assert_eq!(*acc_hash.state(), $typ::zero(), "State after adding and removing the same values did not return to initial state.");
                }

                #[test]
                fn removing_values_must_return_to_original_state() {
                    let mut acc_hash_1 = AccumulativeHash::<$typ>::new();

                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned());
                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned());
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned());

                    let mut combined_values: HashSet<u8> = HashSet::from_iter(SEQUENCE_TO_ADD_1.iter().cloned());
                    SEQUENCE_TO_REMOVE_1.iter().for_each(|&v| { combined_values.remove(&v); });
                    SEQUENCE_TO_ADD_2.iter().for_each(|&v| { combined_values.insert(v); });
                    SEQUENCE_TO_REMOVE_2.iter().for_each(|&v| { combined_values.remove(&v); });
                    dbg!(&combined_values);

                    let mut acc_hash_2 = AccumulativeHash::<$typ>::new();
                    acc_hash_2.add_multiple(combined_values.iter().cloned());

                    assert_eq!(*acc_hash_1.state(), *acc_hash_2.state(), "States do not match after adding and removing sequences versus combined operations.");
                }

                #[test]
                fn merging_states_must_equal_individual_operations() {
                    let mut acc_hash_1 = AccumulativeHash::<$typ>::new();
                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned());

                    let mut acc_hash_2 = AccumulativeHash::<$typ>::new();
                    acc_hash_2.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned());
                    acc_hash_2.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned());

                    acc_hash_1.extend(&acc_hash_2);

                    let mut individual_acc_hash = AccumulativeHash::<$typ>::new();
                    individual_acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned());
                    individual_acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned());
                    individual_acc_hash.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned());
                    individual_acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned());

                    assert_eq!(*acc_hash_1.state(), *individual_acc_hash.state(), "Merged state does not equal individual operations state.");
                }
            }
        };
    }

    test_type!(test_u16::<u16>(
        add_1 = 0x34C4,
        remove_1 = 0xC2CA,
        add_2 = 0xBEBE,
        remove_2 = 0x7ACC
    ));
    test_type!(test_u32::<u32>(
        add_1 = 0xDEE2DA43,
        remove_1 = 0xE7B0E585,
        add_2 = 0xC0516FF0,
        remove_2 = 0x4AF75840
    ));
    test_type!(test_u64::<u64>(
        add_1 = 0x97C3231AEF8AC7C8,
        remove_1 = 0xE62F7B33E88CE12D,
        add_2 = 0xB059A53A13CC2CA2,
        remove_2 = 0x6F428AF403851C01
    ));
    test_type!(test_u128::<u128>(
        add_1 = 0x38AF22CD2CFD6A729755CE3C42316C03,
        remove_1 = 0x9B55A80E93C896FC7AB253CDB11072E0,
        add_2 = 0x171F297C6AC22870A3C6B2DC50BDBCA3,
        remove_2 = 0x3AC8F17636DD11C829BDAC111BA8D724
    ));
    #[cfg(target_pointer_width = "64")]
    test_type!(test_usize::<usize>(
        add_1 = 0x97C3231AEF8AC7C8,
        remove_1 = 0xE62F7B33E88CE12D,
        add_2 = 0xB059A53A13CC2CA2,
        remove_2 = 0x6F428AF403851C01
    ));
    #[cfg(target_pointer_width = "32")]
    test_type!(test_usize::<usize>(
        add_1 = 0xDEE2DA43,
        remove_1 = 0xE7B0E585,
        add_2 = 0xC0516FF0,
        remove_2 = 0x4AF75840
    ));
}
