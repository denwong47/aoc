use super::CountArray;

fn parse_text(input: &str) -> anyhow::Result<Vec<bool>> {
    if !input.starts_with("[") || !input.ends_with("]") {
        anyhow::bail!("Incorrect pattern for Indicators: {:?}", input)
    }

    input[1..input.len() - 1]
        .chars()
        .map(|c| match c {
            '.' => Ok(false),
            '#' => Ok(true),
            _ => anyhow::bail!("Could not parse {} into bool", c),
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Indicators {
    pub(crate) values: CountArray<bool>,
}

impl Indicators {
    pub fn new(values: CountArray<bool>) -> Self {
        Self { values }
    }

    pub fn new_from_input(input: &str) -> anyhow::Result<Self> {
        let values = parse_text(input)?;
        Ok(Self::new(CountArray::from(values)))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.values.iter()
    }
}
