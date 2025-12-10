use super::CountArray;

fn parse_text(input: &str) -> anyhow::Result<Vec<u16>> {
    if !input.starts_with("(") || !input.ends_with(")") {
        anyhow::bail!("Incorrect pattern for Button: {:?}", input)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    pub(crate) index: usize,
    pub(crate) effect: CountArray<bool>,
}

impl Button {
    pub fn new(index: usize, effect: CountArray<bool>) -> Self {
        Self { index, effect }
    }

    pub fn new_from_input(index: usize, input: &str, length: usize) -> anyhow::Result<Self> {
        let mut indices = parse_text(input)?;
        indices.sort();

        if indices.is_empty() {
            anyhow::bail!("Button effect cannot be empty");
        }

        let effect = CountArray::<bool>::from(indices.into_iter().fold(
            vec![false; length],
            |mut acc, idx| {
                acc[idx as usize] = true;
                acc
            },
        ));
        Ok(Self::new(index, effect))
    }

    pub fn len(&self) -> usize {
        self.effect.len()
    }

    pub fn combine<'b>(
        mut buttons: impl Iterator<Item = &'b Button>,
        length: usize,
    ) -> anyhow::Result<CountArray<u16>> {
        buttons.try_fold(CountArray::new(length), |mut acc, button| {
            acc.mut_add(&button.effect)?;
            Ok(acc)
        })
    }
}
