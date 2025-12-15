//! ## Day 2: Gift Shop
//! 
//! You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.
//! 
//! As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.
//! 
//! As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?
//! 
//! They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//! 
//! 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//! 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//! 824824821-824824827,2121212118-2121212124
//! 
//! (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//! 
//! The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
//! 
//! Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
//! 
//! None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)
//! 
//! Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//! 
//!     11-22 has two invalid IDs, 11 and 22.
//!     95-115 has one invalid ID, 99.
//!     998-1012 has one invalid ID, 1010.
//!     1188511880-1188511890 has one invalid ID, 1188511885.
//!     222220-222224 has one invalid ID, 222222.
//!     1698522-1698528 contains no invalid IDs.
//!     446443-446449 has one invalid ID, 446446.
//!     38593856-38593862 has one invalid ID, 38593859.
//!     The rest of the ranges contain no invalid IDs.
//! 
//! Adding up all the invalid IDs in this example produces 1227775554.
//! 
//! What do you get if you add up all of the invalid IDs?
//! 
//! Your puzzle answer was 31839939622.
//! 
//! ## Part Two
//! 
//! The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?
//! 
//! Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.
//! 
//! From the same example as before:
//! 
//!     11-22 still has two invalid IDs, 11 and 22.
//!     95-115 now has two invalid IDs, 99 and 111.
//!     998-1012 now has two invalid IDs, 999 and 1010.
//!     1188511880-1188511890 still has one invalid ID, 1188511885.
//!     222220-222224 still has one invalid ID, 222222.
//!     1698522-1698528 still contains no invalid IDs.
//!     446443-446449 still has one invalid ID, 446446.
//!     38593856-38593862 still has one invalid ID, 38593859.
//!     565653-565659 now has one invalid ID, 565656.
//!     824824821-824824827 now has one invalid ID, 824824824.
//!     2121212118-2121212124 now has one invalid ID, 2121212121.
//! 
//! Adding up all the invalid IDs in this example produces 4174379265.
//! 
//! What do you get if you add up all of the invalid IDs using these new rules?
//! 
//! Your puzzle answer was 41662374059.
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod input;
use input::INPUT;

#[cfg(feature = "profile")]
use std::time::Instant;

const PRIMES: [usize; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn split_input_into_iterables(
    input: &str,
) -> impl Iterator<Item = impl Iterator<Item = u64> + '_> + '_ {
    input.split(',').map(|section| {
        let mut bounds = section
            .split('-')
            .map(|num_str| num_str.parse::<u64>().unwrap());
        let start = bounds.next().unwrap();
        let end = bounds.next().unwrap();
        start..=end
    })
}

fn generate_mask(pattern_length: usize, repeats: usize) -> u64 {
    (0..repeats).fold(0u64, |acc, i| acc + 10u64.pow((i * pattern_length) as u32))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RepeatedPatternInteger {
    pub value: u64,
    pub pattern: u64,
    pub repeats: usize,
}

impl RepeatedPatternInteger {
    pub fn try_from_value_and_repeats(value: u64, repeats: usize) -> Result<Self, anyhow::Error> {
        let digit_count = (value as f32).log10().floor() as usize + 1;
        if !digit_count.is_multiple_of(repeats) {
            return Err(anyhow::anyhow!(
                "Value {} does not have a divisible digit count for pattern length {}",
                value,
                repeats
            ));
        }

        let pattern_length = digit_count / repeats;

        // Special thanks to Mr Kushagra Raina for suggesting the use of a mask.
        let mask = generate_mask(pattern_length, repeats);

        if !value.is_multiple_of(mask) {
            return Err(anyhow::anyhow!(
                "Value {} is not a repeated pattern integer for repeats {}",
                value,
                repeats
            ));
        }

        Ok(Self {
            value,
            pattern: value / mask,
            repeats,
        })
    }

    pub fn try_from_value(value: u64) -> Result<Self, anyhow::Error> {
        let digit_count = (value as f32).log10().floor() as usize + 1;

        PRIMES
            .iter()
            .filter(|&&r| r <= digit_count)
            .find_map(|&r| Self::try_from_value_and_repeats(value, r).ok())
            .ok_or_else(|| anyhow::anyhow!("Value {} is not a repeated pattern integer", value))
    }
}

impl TryFrom<u64> for RepeatedPatternInteger {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::try_from_value(value)
    }
}

struct RepeatedPatternIntegerCounter {
    #[cfg(not(feature = "sum-only"))]
    pub found: Vec<RepeatedPatternInteger>,
    #[cfg(feature = "sum-only")]
    pub sum: u64,
}

impl RepeatedPatternIntegerCounter {
    #[cfg(not(feature = "sum-only"))]
    pub fn new() -> Self {
        Self { found: vec![] }
    }
    #[cfg(feature = "sum-only")]
    pub fn new() -> Self {
        Self { sum: 0 }
    }

    pub fn search_iterable_and_add(&mut self, iterable: impl Iterator<Item = u64>) {
        for item in iterable {
            // Currently only supports R=2
            RepeatedPatternInteger::try_from(item)
                .map(|rpi| {
                    #[cfg(feature = "sum-only")]
                    {
                        self.sum += rpi.value;
                    }
                    #[cfg(not(feature = "sum-only"))]
                    {
                        self.found.push(rpi);
                    }
                })
                .unwrap_or_default();
        }
    }

    pub fn sum(&self) -> u64 {
        #[cfg(feature = "sum-only")]
        {
            self.sum
        }

        #[cfg(not(feature = "sum-only"))]
        {
            self.found.iter().map(|rpi| rpi.value).sum()
        }
    }
}

fn main() {
    #[cfg(feature = "jemalloc")]
    {
        eprintln!("Using jemalloc as the global allocator");
    }
    #[cfg(not(feature = "jemalloc"))]
    {
        eprintln!("Using the default global allocator");
    }

    #[cfg(feature = "profile")]
    let start_time = Instant::now();

    let iterables = split_input_into_iterables(INPUT);

    let mut counter = RepeatedPatternIntegerCounter::new();
    for iterable in iterables {
        #[cfg(feature = "profile-per-loop")]
        let iteration_time = Instant::now();
        counter.search_iterable_and_add(iterable);
        #[cfg(feature = "profile-per-loop")]
        {
            eprintln!("Time taken for iteration: {:?}", iteration_time.elapsed());
        }
    }

    let sum = counter.sum();
    println!("Sum of all repeated pattern integers: {}", sum);

    #[cfg(feature = "profile")]
    {
        eprintln!("Total time taken: {:?}", start_time.elapsed());
    }
}

#[cfg(test)]
mod test_repeated_pattern_integer {
    use super::*;

    macro_rules! create_test {
        ($name:ident::<$r:literal>($value:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let result = RepeatedPatternInteger::try_from_value_and_repeats($value, $r);
                match $expected {
                    Some(RepeatedPatternInteger {
                        value,
                        pattern,
                        repeats,
                    }) => {
                        let rpi = result.expect("Expected Ok result");
                        assert_eq!(rpi.value, value);
                        assert_eq!(rpi.pattern, pattern);
                        assert_eq!(rpi.repeats, repeats);
                    }
                    None => {
                        assert!(result.is_err(), "Expected Err result");
                    }
                }
            }
        };
    }

    create_test!(
        test_valid_1212::<2>(1212) = Some(RepeatedPatternInteger {
            value: 1212,
            pattern: 12,
            repeats: 2,
        })
    );

    create_test!(test_invalid_1234::<2>(1234) = None);

    create_test!(
        test_valid_123123::<2>(123123) = Some(RepeatedPatternInteger {
            value: 123123,
            pattern: 123,
            repeats: 2,
        })
    );

    create_test!(test_invalid_123123::<3>(123123) = None);

    create_test!(
        test_valid_777777::<2>(777777) = Some(RepeatedPatternInteger {
            value: 777777,
            pattern: 777,
            repeats: 2,
        })
    );

    create_test!(
        test_invalid_777777::<3>(777777) = Some(RepeatedPatternInteger {
            value: 777777,
            pattern: 77,
            repeats: 3,
        })
    );
}

#[cfg(test)]
mod test_repeated_pattern_integer_counter {
    use super::*;

    const INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_search_iterable_and_add() {
        let mut counter = RepeatedPatternIntegerCounter::new();
        let iterables = split_input_into_iterables(INPUT);

        for iterable in iterables {
            counter.search_iterable_and_add(iterable);
        }

        let sum = counter.sum();

        assert_eq!(sum, 4174379265);
    }
}

#[cfg(test)]
mod test_generate_mask {
    use super::*;

    macro_rules! create_test {
        ($name:ident(pattern_length=$pattern_length:literal, repeats=$repeats:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let result = generate_mask($pattern_length, $repeats);
                assert_eq!(result, $expected);
            }
        };
    }

    create_test!(test_mask_2x2(pattern_length = 2, repeats = 2) = 101);
    create_test!(test_mask_3x2(pattern_length = 3, repeats = 2) = 1001);
    create_test!(test_mask_2x3(pattern_length = 2, repeats = 3) = 10101);
    create_test!(test_mask_1x5(pattern_length = 1, repeats = 5) = 11111);
}
