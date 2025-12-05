use super::StringRange;

pub trait HasStringRanges {
    fn iter_ranges(&self) -> impl Iterator<Item = &StringRange> + '_;

    // Run through all ranges and see if any contain the value.
    // This tolerates an unordered list of values, but is less efficient.
    fn contains(&self, value: &str) -> bool {
        self.iter_ranges()
            .skip_while(|range| !range.in_range(value))
            .any(|range| range.contains(value))
    }
}

impl HasStringRanges for Vec<StringRange> {
    fn iter_ranges(&self) -> impl Iterator<Item = &StringRange> + '_ {
        self.iter()
    }
}

#[cfg(test)]
mod test_has_string_ranges {
    use super::*;

    const RANGES: &'static [(&'static str, &'static str)] =
        &[("3", "5"), ("10", "14"), ("16", "20"), ("12", "18")];

    macro_rules! create_test {
        ($name:ident($value:literal) == $expected:expr) => {
            #[test]
            fn $name() {
                let mut ranges: Vec<StringRange> = RANGES
                    .iter()
                    .map(|(min, max)| StringRange::new(min, max).expect("Invalid range"))
                    .collect();

                ranges.sort();
                dbg!(&ranges);
                assert_eq!(ranges.contains($value), $expected);
            }
        };
    }

    create_test!(test_contains_4("4") == true);
    create_test!(test_contains_1("1") == false);
    create_test!(test_contains_5("5") == true);
    create_test!(test_contains_8("8") == false);
    create_test!(test_contains_11("11") == true);
    create_test!(test_contains_17("17") == true);
    create_test!(test_contains_32("32") == false);
}
