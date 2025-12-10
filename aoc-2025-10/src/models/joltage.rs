use super::CountArray;

fn parse_text(input: &str) -> anyhow::Result<Vec<u16>> {
    if !input.starts_with("{") || !input.ends_with("}") {
        anyhow::bail!("Incorrect pattern for Joltage: {:?}", input)
    }

    input[1..input.len() - 1]
        .split(",")
        .map(|digits| {
            digits
                .parse::<u16>()
                .map_err(|_| anyhow::anyhow!("Could not parse {} into u16", digits))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Joltage {
    pub(crate) values: CountArray<u16>,
}

impl Joltage {
    pub fn new(values: CountArray<u16>) -> Self {
        Self { values }
    }

    pub fn new_from_input(input: &str) -> anyhow::Result<Self> {
        let values = parse_text(input)?;

        Ok(Self::new(CountArray::from(values)))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u16> {
        self.values.iter()
    }
}
