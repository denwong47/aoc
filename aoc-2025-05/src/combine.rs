use super::StringRange;

pub fn combine_ranges<'r>(ranges: impl Iterator<Item = &'r StringRange>) -> Vec<StringRange> {
    let (mut combined_ranges, mut current_opt) = ranges.fold(
        (Vec::new(), None::<StringRange>),
        |(mut combined, current_opt), range| {
            if let Some(current) = current_opt {
                if let Ok(merged) = StringRange::combine(&current, range) {
                    // Ranges can be combined; update the current range.
                    (combined, Some(merged))
                } else {
                    // Ranges cannot be combined; push the current range and stage a new one.
                    combined.push(current);
                    (combined, Some(range.clone()))
                }
            } else {
                // No current range; start with the first range.
                (combined, Some(range.clone()))
            }
        },
    );

    if let Some(current) = current_opt.take() {
        combined_ranges.push(current);
    }

    combined_ranges
}

#[cfg(test)]
mod test_combine_ranges {
    use super::*;
    use crate::traits::HasStringRanges;

    const RANGES: &'static [(&'static str, &'static str)] = &[
        ("3", "5"),
        ("6", "9"),
        ("8", "11"),
        ("10", "14"),
        ("12", "15"),
        ("16", "20"),
        ("16", "30"),
        ("25", "27"),
    ];

    const EXPECTED: &'static [(&'static str, &'static str)] =
        &[("3", "5"), ("6", "15"), ("16", "30")];

    #[test]
    fn test_combine_ranges() {
        let mut ranges: Vec<StringRange> = RANGES
            .iter()
            .map(|(min, max)| StringRange::new(min, max).expect("Invalid range"))
            .collect();

        ranges.sort();
        dbg!(&ranges);

        let combined = combine_ranges(ranges.iter_ranges());
        let actual = combined
            .iter()
            .map(|r| r.get_print_range())
            .collect::<Vec<_>>();

        let expected = EXPECTED
            .iter()
            .map(|(min, max)| (*min, *max))
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
