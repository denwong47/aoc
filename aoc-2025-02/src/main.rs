mod input;
use input::INPUT;

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
        if digit_count % repeats != 0 {
            return Err(anyhow::anyhow!(
                "Value {} does not have a divisible digit count for pattern length {}",
                value,
                repeats
            ));
        }

        let pattern_length = digit_count / repeats;

        // Special thanks to Mr Kushagra Raina for suggesting the use of a mask.
        let mask = generate_mask(pattern_length, repeats);

        if value % mask != 0 {
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
    pub found: Vec<RepeatedPatternInteger>,
}

impl RepeatedPatternIntegerCounter {
    pub fn new() -> Self {
        Self { found: vec![] }
    }

    pub fn search_iterable_and_add(&mut self, iterable: impl Iterator<Item = u64>) {
        for item in iterable {
            // Currently only supports R=2
            RepeatedPatternInteger::try_from(item)
                .and_then(|rpi| {
                    self.found.push(rpi);
                    Ok(())
                })
                .unwrap_or_default();
        }
    }

    pub fn sum(&self) -> u64 {
        self.found.iter().map(|rpi| rpi.value).sum()
    }
}

fn main() {
    let iterables = split_input_into_iterables(INPUT);

    let mut counter = RepeatedPatternIntegerCounter::new();
    for iterable in iterables {
        counter.search_iterable_and_add(iterable);
    }

    let sum = counter.sum();
    println!("Sum of all repeated pattern integers: {}", sum);
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
