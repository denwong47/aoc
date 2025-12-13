//! The standard model for accumulative hashing, without atomic types.

use crate::{AccumulativeHash, IsAtomicAccumulativeHashType, helpers};

use std::sync::atomic::Ordering;

use num_traits::{WrappingAdd, WrappingSub, Zero};

/// A struct that remembers the state of a hash as data is added and/or removed from it.
#[derive(Debug)]
pub struct AtomicAccumulativeHash<T: IsAtomicAccumulativeHashType> {
    state: T,
}

impl<T: IsAtomicAccumulativeHashType> Clone for AtomicAccumulativeHash<T> {
    /// Create a clone of the current atomic accumulative hash.
    ///
    /// This performs a relaxed load of the current state to initialize the new instance.
    ///
    /// The new instance is cloned by value, and does not share state with the original instance.
    fn clone(&self) -> Self {
        Self::with_state(self.state.to_underlying(Ordering::Relaxed))
    }
}

impl<T: IsAtomicAccumulativeHashType> AtomicAccumulativeHash<T> {
    /// Create a new empty accumulative hash.
    ///
    /// The initial state is equivalent to hashing no values.
    pub fn new() -> Self {
        Self::with_state(T::UnderlyingType::zero().into())
    }

    /// Internal method to add a hashed value to the current state atomically.
    pub fn _raw_op(
        &self,
        is_add: bool,
        hashed_value: T::UnderlyingType,
        success: Ordering,
        failure: Ordering,
    ) -> T::UnderlyingType {
        let mut current_state = self.load(failure);
        loop {
            let new_state = if is_add {
                current_state.wrapping_add(&hashed_value)
            } else {
                current_state.wrapping_sub(&hashed_value)
            };

            match self.state.compare_exchange(
                current_state.into(),
                new_state.into(),
                success,
                failure,
            ) {
                // If the exchange was successful, return the new state, and discard the previous state.
                Ok(_) => return new_state,
                // If the exchange failed, update current_state and retry
                Err(actual) => current_state = actual.into(),
            }
        }
    }

    /// Add a value to the accumulative hash.
    ///
    /// This does not guarantee that the value was never added before; it will simply
    /// add the hashed value to the current state.
    ///
    /// This means that adding the same value multiple times will affect the hash state
    /// accordingly.
    ///
    /// This method uses atomic operations to ensure thread safety; it takes two
    /// [`Ordering`] parameters to specify the memory ordering for success and failure cases.
    /// ``failure`` is also used when loading the current state for the first time.
    ///
    /// It guarantees that the returned state is the result of the addition, and no race
    /// conditions can occur, at the cost of potential retries if other threads modify
    /// the state concurrently.
    pub fn add<S: Into<T::UnderlyingType>>(
        &self,
        value: S,
        success: Ordering,
        failure: Ordering,
    ) -> T::UnderlyingType {
        let value_as_underlying = value.into();
        let hashed = helpers::hash::<T::UnderlyingType, _>(value_as_underlying);

        self._raw_op(true, hashed, success, failure)
    }

    /// Remove a value from the accumulative hash.
    ///
    /// This does not guarantee that the value was previously added;
    /// it will simply subtract the hashed value from the current state.
    ///
    /// This means that removing a value that was never added may lead to
    /// undetermined behavior; it can be fixed by re-adding the value later,
    /// but the intermediate state may not be valid.
    ///
    /// This method uses atomic operations to ensure thread safety; it takes two
    /// [`Ordering`] parameters to specify the memory ordering for success and failure cases.
    /// ``failure`` is also used when loading the current state for the first time.
    ///
    /// It guarantees that the returned state is the result of the removal, and no race
    /// conditions can occur, at the cost of potential retries if other threads modify
    /// the state concurrently.
    pub fn remove<S: Into<T::UnderlyingType>>(
        &self,
        value: S,
        success: Ordering,
        failure: Ordering,
    ) -> T::UnderlyingType {
        let value_as_underlying = value.into();
        let hashed = helpers::hash::<T::UnderlyingType, _>(value_as_underlying);

        self._raw_op(false, hashed, success, failure)
    }

    /// Add multiple values to the accumulative hash.
    ///
    /// This does not guarantee that the values were never added before; it will simply
    /// add the hashed values to the current state.
    ///
    /// This means that adding the same value multiple times will affect the hash state
    /// accordingly.
    ///
    /// This method uses atomic operations to ensure thread safety; it takes two
    /// [`Ordering`] parameters to specify the memory ordering for success and failure cases.
    /// ``failure`` is also used when loading the current state for the first time.
    ///
    /// It guarantees that the returned state is the result of the addition, and no race
    /// conditions can occur, at the cost of potential retries if other threads modify
    /// the state concurrently.
    pub fn add_multiple<S: Into<T::UnderlyingType>, I: IntoIterator<Item = S>>(
        &self,
        values: I,
        success: Ordering,
        failure: Ordering,
    ) -> T::UnderlyingType {
        // Pre-calculate the combined hash of all values to added first, so that we can reduce the race window
        // between loading the current state and updating it.
        let combined_state = AccumulativeHash::<T::UnderlyingType>::from(values).into_state();

        self._raw_op(true, combined_state, success, failure)
    }

    /// Remove multiple values from the accumulative hash.
    ///
    /// This does not guarantee that the values were previously added;
    /// it will simply subtract the hashed values from the current state.
    ///
    /// This means that removing values that were never added may lead to
    /// undetermined behavior; it can be fixed by re-adding the values later,
    /// but the intermediate state may not be valid.
    ///
    /// This method uses atomic operations to ensure thread safety; it takes two
    /// [`Ordering`] parameters to specify the memory ordering for success and failure cases.
    /// ``failure`` is also used when loading the current state for the first time.
    ///
    /// It guarantees that the returned state is the result of the removal, and no race
    /// conditions can occur, at the cost of potential retries if other threads modify
    /// the state concurrently.
    pub fn remove_multiple<S: Into<T::UnderlyingType>, I: IntoIterator<Item = S>>(
        &self,
        values: I,
        success: Ordering,
        failure: Ordering,
    ) -> T::UnderlyingType {
        // Pre-calculate the combined hash of all values to removed first, so that we can reduce the race window
        // between loading the current state and updating it.
        let combined_state = AccumulativeHash::<T::UnderlyingType>::from(values).into_state();

        self._raw_op(false, combined_state, success, failure)
    }

    /// Create a new accumulative hash with an initial state.
    pub fn with_state(state: T::UnderlyingType) -> Self {
        Self {
            state: state.into(),
        }
    }

    /// Get the current state of the accumulative hash.
    pub fn state(&self) -> &T {
        &self.state
    }

    /// Load the current state of the accumulative hash as the underlying type.
    pub fn load(&self, order: Ordering) -> T::UnderlyingType {
        self.state.to_underlying(order)
    }

    /// Extend the current accumulative hash with another accumulative hash.
    pub fn extend(&self, other: &Self, success: Ordering, failure: Ordering) {
        let other_state = other.load(failure);

        self._raw_op(true, other_state, success, failure);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[cfg(target_has_atomic_load_store = "16")]
    use std::sync::atomic::AtomicU16;
    // #[cfg(target_has_atomic_load_store = "32")]
    use std::sync::atomic::AtomicU32;
    // #[cfg(target_has_atomic_load_store = "64")]
    use std::sync::atomic::AtomicU64;
    // #[cfg(any(target_has_atomic_load_store = "64", target_has_atomic_load_store = "32"))]
    #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
    use std::sync::atomic::AtomicUsize;

    const SEQUENCE_TO_ADD_1: &'static [u8] = &[1, 2, 4, 8, 16, 32, 64, 128];
    const SEQUENCE_TO_REMOVE_1: &'static [u8] = &[1, 4, 8, 64];
    const SEQUENCE_TO_ADD_2: &'static [u8] = &[3, 6, 9, 12, 15];
    const SEQUENCE_TO_REMOVE_2: &'static [u8] = &[2, 6, 12];

    const LOAD_ORDER: Ordering = Ordering::Acquire;
    const STORE_ORDER: Ordering = Ordering::Release;

    macro_rules! test_type {
        ($name:ident::<$typ:ident>(add_1=$add_1:literal, remove_1=$remove_1:literal, add_2=$add_2:literal, remove_2=$remove_2:literal)) => {
            mod $name {
                use super::*;
                use num_traits::Zero;
                use std::collections::HashSet;

                #[test]
                fn sequential_add_must_equal_multiple_add() {
                    let acc_hash_seq = AtomicAccumulativeHash::<$typ>::new();
                    for &value in SEQUENCE_TO_ADD_1.iter() {
                        acc_hash_seq.add(value, STORE_ORDER, LOAD_ORDER);
                    }
                    let state_seq = acc_hash_seq.load(LOAD_ORDER);

                    let acc_hash_multi = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_multi.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    let state_multi = acc_hash_multi.load(LOAD_ORDER);

                    assert_eq!(state_seq, state_multi, "Sequential add and multiple add states do not match.");
                }

                #[test]
                fn sequential_add_must_equal_to_unordered_add() {
                    let acc_hash_seq = AtomicAccumulativeHash::<$typ>::new();
                    let state_seq = acc_hash_seq.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    let acc_hash_unordered = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_unordered.add_multiple(SEQUENCE_TO_ADD_1.iter().rev().cloned(), STORE_ORDER, LOAD_ORDER);
                    let state_unordered = acc_hash_unordered.load(LOAD_ORDER);

                    assert_eq!(state_seq, state_unordered, "Sequential add and unordered add states do not match.");
                }

                #[test]
                fn sequential_remove_must_equal_multiple_remove() {
                    let acc_hash_seq = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_seq.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    let acc_hash_unordered = acc_hash_seq.clone();

                    let state_seq = acc_hash_seq.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    let state_multi = acc_hash_unordered.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().rev().cloned(), STORE_ORDER, LOAD_ORDER);

                    assert_eq!(state_seq, state_multi, "Sequential remove and multiple remove states do not match.");
                }

                #[test]
                fn sequential_add_and_remove() {
                    let acc_hash = AtomicAccumulativeHash::<$typ>::new();

                    let expected_after_add_1: <$typ as IsAtomicAccumulativeHashType>::UnderlyingType = $add_1;
                    let actual_after_add_1 = acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    assert!(actual_after_add_1 == expected_after_add_1, "Hashes not matching after first addition: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_add_1, stringify!(==), expected_after_add_1);

                    let expected_after_remove_1: <$typ as IsAtomicAccumulativeHashType>::UnderlyingType = $remove_1;
                    let actual_after_remove_1 = acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    assert!(actual_after_remove_1 == expected_after_remove_1, "Hashes not matching after first removal: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_remove_1, stringify!(==), expected_after_remove_1);
                    let expected_after_add_2: <$typ as IsAtomicAccumulativeHashType>::UnderlyingType = $add_2;
                    let actual_after_add_2 = acc_hash.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    assert!(actual_after_add_2 == expected_after_add_2, "Hashes not matching after second addition: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_add_2, stringify!(==), expected_after_add_2);

                    let expected_after_remove_2: <$typ as IsAtomicAccumulativeHashType>::UnderlyingType = $remove_2;
                    let actual_after_remove_2 = acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    assert!(actual_after_remove_2 == expected_after_remove_2, "Hashes not matching after second removal: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", actual_after_remove_2, stringify!(==), expected_after_remove_2);
                }

                #[test]
                fn adding_and_removing_same_values_must_return_to_initial_state() {
                    let acc_hash = AtomicAccumulativeHash::<$typ>::new();

                    acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash.remove_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    assert_eq!(acc_hash.load(LOAD_ORDER), <$typ as IsAtomicAccumulativeHashType>::UnderlyingType::zero(), "State after adding and removing the same values did not return to initial state.");
                }

                #[test]
                fn removing_values_must_return_to_original_state() {
                    let acc_hash_1 = AtomicAccumulativeHash::<$typ>::new();

                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    let mut combined_values: HashSet<u8> = HashSet::from_iter(SEQUENCE_TO_ADD_1.iter().cloned());
                    SEQUENCE_TO_REMOVE_1.iter().for_each(|&v| { combined_values.remove(&v); });
                    SEQUENCE_TO_ADD_2.iter().for_each(|&v| { combined_values.insert(v); });
                    SEQUENCE_TO_REMOVE_2.iter().for_each(|&v| { combined_values.remove(&v); });
                    dbg!(&combined_values);

                    let acc_hash_2 = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_2.add_multiple(combined_values.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    assert_eq!(acc_hash_1.load(LOAD_ORDER), acc_hash_2.load(LOAD_ORDER), "States do not match after adding and removing sequences versus combined operations.");
                }

                #[test]
                fn merging_states_must_equal_individual_operations() {
                    let acc_hash_1 = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_1.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash_1.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    let  acc_hash_2 = AtomicAccumulativeHash::<$typ>::new();
                    acc_hash_2.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    acc_hash_2.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    acc_hash_1.extend(&acc_hash_2, STORE_ORDER, LOAD_ORDER);

                    let individual_acc_hash = AtomicAccumulativeHash::<$typ>::new();
                    individual_acc_hash.add_multiple(SEQUENCE_TO_ADD_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    individual_acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_1.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    individual_acc_hash.add_multiple(SEQUENCE_TO_ADD_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);
                    individual_acc_hash.remove_multiple(SEQUENCE_TO_REMOVE_2.iter().cloned(), STORE_ORDER, LOAD_ORDER);

                    assert_eq!(acc_hash_1.load(LOAD_ORDER), individual_acc_hash.load(LOAD_ORDER), "Merged state does not equal individual operations state.");
                }
            }
        };
    }

    test_type!(test_u16::<AtomicU16>(
        add_1 = 0x34C4,
        remove_1 = 0xC2CA,
        add_2 = 0xBEBE,
        remove_2 = 0x7ACC
    ));
    test_type!(test_u32::<AtomicU32>(
        add_1 = 0xDEE2DA43,
        remove_1 = 0xE7B0E585,
        add_2 = 0xC0516FF0,
        remove_2 = 0x4AF75840
    ));
    test_type!(test_u64::<AtomicU64>(
        add_1 = 0x97C3231AEF8AC7C8,
        remove_1 = 0xE62F7B33E88CE12D,
        add_2 = 0xB059A53A13CC2CA2,
        remove_2 = 0x6F428AF403851C01
    ));
    #[cfg(target_pointer_width = "64")]
    test_type!(test_usize::<AtomicUsize>(
        add_1 = 0x97C3231AEF8AC7C8,
        remove_1 = 0xE62F7B33E88CE12D,
        add_2 = 0xB059A53A13CC2CA2,
        remove_2 = 0x6F428AF403851C01
    ));

    #[cfg(target_pointer_width = "32")]
    test_type!(test_usize::<AtomicUsize>(
        add_1 = 0xDEE2DA43,
        remove_1 = 0xE7B0E585,
        add_2 = 0xC0516FF0,
        remove_2 = 0x4AF75840
    ));
}
