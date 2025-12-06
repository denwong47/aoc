use anyhow::anyhow;

use crate::{AddToBuffer, BufferedLineReader, Operator};

pub struct Orchestrator<'s, B: AddToBuffer> {
    numeric_buffers: Vec<BufferedLineReader<'s, B>>,
    operator_buffer: BufferedLineReader<'s, Operator>,
}

impl<'s, B: AddToBuffer> Orchestrator<'s, B> {
    /// Create an Orchestrator from an iterator over lines of text.
    pub fn from_lines(lines: impl Iterator<Item = &'s str>) -> anyhow::Result<Self> {
        let mut numeric_lines = Vec::new();
        let mut operator_lines = Vec::new();

        lines.for_each(|line| match line.chars().next() {
            Some('0'..='9') => numeric_lines.push(line),
            Some(' ') => numeric_lines.push(line),
            Some('+') | Some('*') => operator_lines.push(line),
            _ => {}
        });

        if operator_lines.len() != 1 {
            anyhow::bail!(
                "Expected exactly one operator line, found {}",
                operator_lines.len()
            );
        }

        let numeric_buffers = numeric_lines
            .into_iter()
            .map(|line| BufferedLineReader::new(line))
            .collect::<Vec<_>>();
        let operator_buffer = BufferedLineReader::new(operator_lines[0]);

        Ok(Self {
            numeric_buffers,
            operator_buffer,
        })
    }

    /// Create an Orchestrator from a block of text.
    pub fn from_text(text: &'s str) -> anyhow::Result<Self> {
        Self::from_lines(text.lines())
    }
}

impl<'s> Orchestrator<'s, u16> {
    /// Parse each segment horizonally as a number, then operate on them vertically.
    pub fn horizontal_process(mut self) -> anyhow::Result<u128> {
        // We can't use `try_fold` because we need to &mut operator_buffer twice.
        let mut acc = 0_u128;
        loop {
            let operator_char = self.operator_buffer.advance()?;
            let all_digits: Vec<Option<char>> = {
                self.numeric_buffers
                    .iter_mut()
                    .map(|buf| buf.advance())
                    .collect::<anyhow::Result<Vec<Option<char>>>>()
            }?;

            let is_exhausted = operator_char.is_none() && all_digits.iter().all(|c| c.is_none());
            if is_exhausted
                || operator_char.is_some_and(|c| c.is_whitespace())
                    && all_digits
                        .iter()
                        .all(|oc| oc.is_some_and(|c| c.is_whitespace()))
            {
                #[cfg(feature = "trace")]
                eprintln!(
                    "Processing segment with all whitespaces, currently accumulated: {}",
                    acc
                );

                // If everything yielded a whitespace, then we know that we have got the columns we
                // needed. Let's start processing.
                let numbers = self
                    .numeric_buffers
                    .iter_mut()
                    .map(|buf| buf.yield_buffer());

                let operator = self.operator_buffer.yield_buffer();

                acc = acc
                    .checked_add(operator.operate_on(numbers)?)
                    .ok_or_else(|| anyhow::anyhow!("Overflow occurred during accumulation"))?;

                if is_exhausted {
                    break;
                }
            }
        }

        Ok(acc)
    }
}

impl<'s> Orchestrator<'s, Vec<Option<u8>>> {
    /// Parse each segment horizonally as a number, then operate on them vertically.
    pub fn vertical_process(mut self) -> anyhow::Result<u128> {
        // We can't use `try_fold` because we need to &mut operator_buffer twice.
        let mut acc = 0_u128;
        loop {
            let operator_char = self.operator_buffer.advance()?;
            let all_digits: Vec<Option<char>> = {
                self.numeric_buffers
                    .iter_mut()
                    .map(|buf| buf.advance())
                    .collect::<anyhow::Result<Vec<Option<char>>>>()
            }?;

            let is_exhausted = operator_char.is_none() && all_digits.iter().all(|c| c.is_none());
            if is_exhausted
                || operator_char.is_some_and(|c| c.is_whitespace())
                    && all_digits
                        .iter()
                        .all(|oc| oc.is_some_and(|c| c.is_whitespace()))
            {
                #[cfg(feature = "trace")]
                eprintln!(
                    "Processing segment with all whitespaces, currently accumulated: {}",
                    acc
                );

                let numbers = self
                    .numeric_buffers
                    .iter_mut()
                    .try_fold(None, |mut vec, buf| {
                        let mut digits = buf.yield_buffer();

                        // If the line is not exhausted, we must have inserted a trailing None for teh
                        // separator (i.e. the whitespace we were checking for above). We need to pop it
                        // off to avoid messing up multiplication.
                        if !is_exhausted {
                            digits.pop_if(|d| d.is_none());
                        }

                        // Lazy init because we don't know how many numbers there are yet.
                        if vec.is_none() {
                            vec = Some(vec![0_u16; digits.len()]);
                        }

                        digits.iter().enumerate().try_for_each(|(idx, digit)| {
                            if digit.is_none() {
                                return Ok(());
                            }

                            let existing_number = vec.as_ref().expect("Unreachable")[idx];
                            vec.as_mut().expect("Unreachable")[idx] = existing_number
                                .checked_mul(10)
                                .and_then(|v| v.checked_add(digit.unwrap() as u16))
                                .ok_or_else(|| {
                                    anyhow!(
                                        "Overflow when shifting number during vertical processing"
                                    )
                                })?;

                            Ok::<_, anyhow::Error>(())
                        })?;

                        Ok::<_, anyhow::Error>(vec)
                    })?
                    .expect("Unreachable: vec should be initialized");

                let operator = self.operator_buffer.yield_buffer();

                acc = acc
                    .checked_add(operator.operate_on(numbers.into_iter())?)
                    .ok_or_else(|| anyhow::anyhow!("Overflow occurred during accumulation"))?;

                if is_exhausted {
                    break;
                }
            }
        }

        Ok(acc)
    }
}

#[cfg(test)]
mod test_orchestrator {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_horizontal_process() {
        let orchestrator = Orchestrator::from_text(TEST_INPUT)
            .expect("Failed to create orchestrator from test input");

        let result = orchestrator
            .horizontal_process()
            .expect("Failed to process horizontally");

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_vertical_process() {
        let orchestrator = Orchestrator::from_text(TEST_INPUT)
            .expect("Failed to create orchestrator from test input");

        let result = orchestrator
            .vertical_process()
            .expect("Failed to process vertically");

        assert_eq!(result, 3263827);
    }
}
