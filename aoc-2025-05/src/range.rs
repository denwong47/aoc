fn pad_string(value: &str, width: usize) -> String {
    format!("{:0>width$}", value, width = width)
}

/// A range of strings, defined by a start and end string (inclusive).
///
/// For the purposes of this challenge, all values are well within the [`u64`] range,
/// so we could have done this whole challenge with [`std::ops::RangeInclusive`], but this
/// implementation is more general and can handle infinitely large strings (provided that
/// each of their range size is less than or equal to [`i64::MAX`]) as well as
/// non-numeric strings (though untested).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringRange {
    min_len: usize,
    max_len: usize,
    start: String,
    end: String,
}

impl StringRange {
    pub fn new(start: &str, end: &str) -> anyhow::Result<Self> {
        let min_len = start.len().min(end.len());
        let max_len = start.len().max(end.len());
        let start_padded = pad_string(start, max_len);
        let end_padded = pad_string(end, max_len);
        if end_padded < start_padded {
            anyhow::bail!("end must be greater than or equal to start");
        }
        Ok(Self {
            min_len,
            max_len,
            start: start_padded,
            end: end_padded,
        })
    }

    /// Check if the length of the value is within the min and max length of the range;
    /// if its not, there is no point in checking further.
    pub fn in_range(&self, value: &str) -> bool {
        let candidate_len = value.len();
        candidate_len >= self.min_len && candidate_len <= self.max_len
    }

    pub fn contains(&self, value: &str) -> bool {
        if !self.in_range(value) {
            #[cfg(feature = "trace")]
            {
                eprintln!(
                    "Value: '{value}' length not in range: {}-{}",
                    self.min_len, self.max_len
                );
            }
            return false;
        }
        let value_padded = format!("{:0>width$}", value, width = self.max_len);

        #[cfg(feature = "trace")]
        {
            eprintln!(
                "Comparing value: '{value_padded}' with range: '{:?}' - '{:?}'",
                self.start, self.end
            );
        }
        // Inclusive range check
        value_padded >= self.start && value_padded <= self.end
    }

    pub fn get_size(&self) -> usize {
        let unchecked = self
            .start
            .chars()
            .rev()
            .zip(self.end.chars().rev())
            .enumerate()
            .fold(0_i64, |acc, (idx, (s_char, e_char))| {
                // This tolerates non-digit characters, but this is currently untested.
                let diff = e_char as i64 - s_char as i64;
                acc.checked_add(diff * 10_i64.pow(idx as u32))
                    .expect("Range size overflowed i64; range too large to compute size")
            });

        assert!(unchecked >= 0, "Range size must be positive");
        (unchecked + 1) as usize
    }

    /// Static method to combine two ranges into one encompassing range if possible.
    pub fn combine(this: &Self, that: &Self) -> anyhow::Result<Self> {
        let sorted = if this < that {
            (this, that)
        } else {
            (that, this)
        };

        let max_len = sorted.0.max_len.max(sorted.1.max_len);
        match (
            pad_string(&sorted.0.start, max_len),
            pad_string(&sorted.0.end, max_len),
            pad_string(&sorted.1.start, max_len),
            pad_string(&sorted.1.end, max_len),
        ) {
            (start_a, end_a, start_b, end_b) if start_b >= start_a && end_a >= start_b => {
                // Ranges overlap or are contiguous
                let new_start = start_a;
                let new_end = end_a.max(end_b);
                StringRange::new(&new_start, &new_end)
            }
            // Welp turns out there are no other cases here, clippy be mad lol
            _ => anyhow::bail!("Ranges do not overlap and cannot be combined"),
        }
    }

    #[cfg(test)]
    pub fn get_print_range(&self) -> (&str, &str) {
        (
            self.start.trim_start_matches("0"),
            self.end.trim_start_matches("0"),
        )
    }
}

impl PartialOrd for StringRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(<Self as Ord>::cmp(self, other))
    }
}

impl Ord for StringRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_len = self.max_len.max(other.max_len);
        self.min_len
            .cmp(&other.min_len)
            .then_with(
                // We know that the min lengths are equal here, but one of them
                // could still be padded to reach the max length, so we need to
                // compare the starts up to the min length.
                || pad_string(&self.start, max_len).cmp(&pad_string(&other.start, max_len)),
            )
            .then_with(
                // Now that we know the starts are equal, and the max lengths are equal,
                // we can compare the ends lexicographically.
                || self.max_len.cmp(&other.max_len),
            )
            .then_with(|| self.end.cmp(&other.end))
    }
}

#[cfg(test)]
mod test_struct_contains {
    use super::*;

    macro_rules! create_test {
        ($name:ident($candidate:literal in ($start:literal, $end:literal)) = $expected:expr) => {
            #[test]
            fn $name() {
                let range = StringRange::new($start, $end).unwrap();
                let result = range.contains($candidate);
                assert_eq!(result, $expected);
            }
        };
    }

    create_test!(test_mid_range("4" in ("3", "5")) = true);
    create_test!(test_below_range("2" in ("3", "5")) = false);
    create_test!(test_above_range("6" in ("3", "5")) = false);
    create_test!(test_different_lengths_below("99" in ("100", "200")) = false);
    create_test!(test_different_lengths_within("150" in ("10", "200")) = true);
    create_test!(test_different_lengths_above("201" in ("10", "200")) = false);
    create_test!(test_edge_start("100" in ("100", "200")) = true);
    create_test!(test_edge_end("200" in ("100", "200")) = true);
    create_test!(test_exact_match("123" in ("123", "123")) = true);
}

#[cfg(test)]
mod test_struct_sort {
    use super::*;

    const RANGES: &'static [(&'static str, &'static str)] = &[
        ("1", "10"),
        ("100", "200"),
        ("20", "30"),
        ("2", "3"),
        ("3000", "4000"),
        ("3", "5"),
        ("8", "11"),
        ("10", "14"),
        ("16", "20"),
        ("12", "18"),
        ("1", "1000"),
        ("20", "29"),
    ];

    #[test]
    fn test_sorting() {
        let ranges = RANGES
            .iter()
            .map(|(start, end)| StringRange::new(start, end).expect("Invalid range"))
            .collect::<Vec<_>>();

        let mut sorted_ranges = ranges.clone();
        sorted_ranges.sort();

        let expected_order = vec![
            ("1", "10"),
            ("1", "1000"),
            ("2", "3"),
            ("3", "5"),
            ("8", "11"),
            ("10", "14"),
            ("12", "18"),
            ("16", "20"),
            ("20", "29"),
            ("20", "30"),
            ("100", "200"),
            ("3000", "4000"),
        ];

        let actual_order = sorted_ranges
            .iter()
            .map(|r| r.get_print_range())
            .collect::<Vec<_>>();

        assert_eq!(actual_order, expected_order);
    }
}

#[cfg(test)]
mod test_struct_size {
    use super::*;

    macro_rules! create_size_test {
        ($name:ident($start:literal, $end:literal) == $expected:expr) => {
            #[test]
            fn $name() {
                let range = StringRange::new($start, $end).unwrap();
                let size = range.get_size();
                assert_eq!(size, $expected);
            }
        };
    }

    create_size_test!(test_size_3_3("3", "3") == 1);
    create_size_test!(test_size_3_5("3", "5") == 3);
    create_size_test!(test_size_10_14("10", "14") == 5);
    create_size_test!(test_size_100_200("100", "200") == 101);
    create_size_test!(test_size_1_1000("1", "1000") == 1000);
    create_size_test!(test_size_99_101("99", "101") == 3);
}

#[cfg(test)]
mod test_struct_combine {
    use super::*;

    type TestResult = Result<(&'static str, &'static str), ()>;

    macro_rules! create_test {
        ($name:ident(($start_a:literal, $end_a:literal), ($start_b:literal, $end_b:literal) => $expected:expr) ) => {
            #[test]
            fn $name() {
                let range_a = StringRange::new($start_a, $end_a).expect("Invalid range");
                let range_b = StringRange::new($start_b, $end_b).expect("Invalid range");
                let combined = StringRange::combine(&range_a, &range_b);
                if let Ok((expected_start, expected_end)) = $expected {
                    let actual = combined.expect("Expected ranges to combine");
                    let (combined_start, combined_end) = actual.get_print_range();
                    assert_eq!(combined_start, expected_start);
                    assert_eq!(combined_end, expected_end);
                } else {
                    assert!(combined.is_err(), "Expected ranges not to combine");
                }
            }
        };
    }

    create_test!(test_overlap_start(("10", "20"), ("15", "25") => TestResult::Ok(("10", "25"))));
    create_test!(test_overlap_end(("15", "25"), ("10", "20") => TestResult::Ok(("10", "25"))));
    create_test!(test_contiguous_end_start(("10", "20"), ("21", "30") => TestResult::Err(())));
    create_test!(test_contiguous_start_end(("21", "30"), ("10", "20") => TestResult::Err(())));
    create_test!(test_fully_contained(("10", "30"), ("15", "25") => TestResult::Ok(("10", "30"))));
    create_test!(test_full_contained_with_overlapping_start(("10", "25"), ("10", "30") => TestResult::Ok(("10", "30"))));
    create_test!(test_full_contained_with_overlapping_end(("15", "30"), ("10", "30") => TestResult::Ok(("10", "30"))));
    create_test!(test_no_overlap(("10", "15"), ("20", "25") => TestResult::Err(())));
}
