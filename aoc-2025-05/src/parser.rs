use super::StringRange;

pub struct ParsedInput {
    pub ranges: Vec<StringRange>,
    pub values: Vec<String>,
}

pub fn parse_input(input: &str) -> ParsedInput {
    let mut ranges = Vec::new();
    let mut values = Vec::new();
    input.lines().for_each(|line| match line.split_once('-') {
        Some((min, max)) => {
            ranges.push(StringRange::new(min.trim(), max.trim()).expect("Invalid range"));
        }
        None => {
            let value = line.trim();
            if !value.is_empty() {
                values.push(value.to_string());
            }
        }
    });

    ranges.sort();
    ParsedInput { ranges, values }
}

#[cfg(test)]
mod test_parse_input {
    use super::*;

    const TEST_INPUT: &'static str = "
    3-5
    10-14
    16-20
    12-18
    
    1
    5
    8
    11
    17
    32
    ";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(TEST_INPUT);
        let expected_ranges = vec![
            StringRange::new("3", "5").unwrap(),
            StringRange::new("10", "14").unwrap(),
            StringRange::new("12", "18").unwrap(),
            StringRange::new("16", "20").unwrap(),
        ];
        assert_eq!(parsed.ranges, expected_ranges);
        assert_eq!(
            parsed.values,
            vec![
                "1".to_string(),
                "5".to_string(),
                "8".to_string(),
                "11".to_string(),
                "17".to_string(),
                "32".to_string(),
            ]
        );
    }
}
