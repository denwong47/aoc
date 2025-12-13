//! Helper types and functions for accumulative hashing.

use crate::IsAccumulativeHashType;

/// Hash a value into a state where it can be wrapping added or removed from the
/// accumulative hash.
pub fn hash<T: IsAccumulativeHashType + From<S>, S>(value: S) -> T {
    let mut z = (T::from(value)).wrapping_add(&T::SEED);
    z = (z ^ (z >> T::SHIFT_CONSTANTS[0])).wrapping_mul(&T::MULTIPLIER_CONSTANTS[0]);
    z = (z ^ (z >> T::SHIFT_CONSTANTS[1])).wrapping_mul(&T::MULTIPLIER_CONSTANTS[1]);
    z ^ (z >> T::SHIFT_CONSTANTS[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn difference<T: std::ops::Sub<Output = T> + Ord>(a: T, b: T) -> T {
        if a > b { a - b } else { b - a }
    }

    macro_rules! test_type {
        ($name:ident::<$typ:ty>($($inner_name:ident($value:literal) => $expected:expr),+)) => {
            mod $name {
                use super::*;
                use std::collections::HashSet;

                macro_rules! create_test {
                    ($test_name:ident::<$test_typ:ty>($input:literal) $op:tt $output:expr) => {
                        #[test]
                        fn $test_name() {
                            let hashed = hash::<$test_typ, _>($input);
                            let expected: $test_typ = $output;
                            assert!(hashed $op expected, "Failed condition: \x1b[31moutput\x1b[0m \u{2192} \x1b[34m\x1b[1m0x{:X}\x1b[22m {} \x1b[1m0x{:X}\x1b[0m \u{2190} \x1b[32mexpected\x1b[0m", hashed, stringify!($op), expected);
                        }
                    };
                }

                $(
                    mod $inner_name {
                        use super::*;

                        create_test!(equality::<$typ>($value) == $expected);
                        create_test!(hash_must_equal_regardless_of_incoming_type::<$typ>($value) == hash::<$typ, _>($value as u16));
                    }
                )+
                create_test!(hashes_must_be_deterministic::<$typ>(0_u8) == hash::<$typ, _>(0_u8));
                create_test!(hashes_must_not_be_algorithmic_1::<$typ>(1_u8) != difference(hash::<$typ, _>(4_u8), hash::<$typ, _>(3_u8)));
                create_test!(hashes_must_not_be_algorithmic_2::<$typ>(32767_u16) != difference(hash::<$typ, _>(65535_u16), hash::<$typ, _>(32768_u16)));

                #[test]
                fn hashes_must_not_be_in_order() {
                    let order_established = (0..255_u8).try_fold(0 as $typ, |acc, x| {
                        let current_hash = hash::<$typ, _>(x);
                        if current_hash > acc {
                            Ok(current_hash)
                        } else {
                            Err(())
                        }
                    });

                    assert!(order_established.is_err(), "Hashes appear to be in order, which should not happen.");
                }

                /// This test is quite harsh for [`u16`], as it literally requires
                /// every possible hash to be unique for all possible inputs.
                ///
                /// For larger types like [`u32`], [`u64`], and [`u128`], this test
                /// is less strict, as the probability of collisions is much lower.
                ///
                /// Nonetheless, this test is important to ensure the quality of the
                /// hashing algorithm.
                #[test]
                fn hashes_must_be_unique() {
                    let mut seen_hashes: HashSet<$typ> = HashSet::with_capacity(65536);

                    for x in 0..65535_u16 {
                        let current_hash = hash::<$typ, _>(x);
                        assert!(seen_hashes.insert(current_hash), "Hash collision detected for input value \x1b[1m{}\x1b[0m resulting in hash \x1b[1m0x{:X}\x1b[0m", x, current_hash);

                    }
                }
            }
        };
    }

    test_type!(test_u16::<u16>(hash_1(1_u8) => 0x514A, hash_2(2_u8) => 0x99CE));
    test_type!(test_u32::<u32>(hash_1(1_u8) => 0x8FC49058, hash_2(2_u8) => 0x77145E31));
    test_type!(test_u64::<u64>(hash_1(1_u8) => 0x1E432F0FA8382AF8, hash_2(2_u8) => 0xB597E69E2146AEF1));
    test_type!(test_u128::<u128>(hash_1(1_u8) => 0xD78F4A68D49E28E54D77B489467D2DA8, hash_2(2_u8) => 0x2370B5AE40918D1C047C3BD11E39E9C));
}
