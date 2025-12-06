use super::AddToBuffer;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Operator {
    Add,
    Multiply,
    #[default]
    Undefined,
}

impl Operator {
    pub fn operate_on(&self, mut items: impl Iterator<Item = u16>) -> anyhow::Result<u128> {
        items
            .try_fold(None, |acc, item| -> anyhow::Result<Option<u128>> {
                #[cfg(feature = "trace")]
                eprintln!("Operating: {:?} with acc={:?} and item={}", self, acc, item);
                match self {
                    Operator::Add => acc
                        .or(Some(0_u128))
                        .map(|acc| acc.checked_add(item as u128))
                        .ok_or_else(|| anyhow::anyhow!("Overflow in addition")),
                    Operator::Multiply => acc
                        .or(Some(1_u128))
                        .map(|acc| acc.checked_mul(item as u128))
                        .ok_or_else(|| anyhow::anyhow!("Overflow in multiplication")),
                    Operator::Undefined => {
                        anyhow::bail!("Cannot operate with undefined operator")
                    }
                }
            })
            .map(|result_opt| result_opt.unwrap_or_default())
    }
}

impl AddToBuffer for Operator {
    fn add_to_buffer(&mut self, input: char) -> anyhow::Result<char> {
        if input == ' ' {
            return Ok(input);
        }

        if self != &Operator::Undefined {
            anyhow::bail!(
                "Operator already defined as {:?}, cannot add {:?}",
                self,
                input
            );
        }

        match input {
            '+' => {
                *self = Operator::Add;
            }
            '*' => {
                *self = Operator::Multiply;
            }
            _ => {
                anyhow::bail!("Invalid operator character: {:?}", input);
            }
        }

        Ok(input)
    }
}

#[cfg(test)]
mod test_add_operator {
    use super::*;
    use crate::AddToBuffer;

    macro_rules! create_test {
        ($name:ident($operator:expr, $char:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let mut operator = $operator;
                let result = operator.add_to_buffer($char);

                let expected: anyhow::Result<Operator> = $expected;
                match expected {
                    Ok(expected_char) => {
                        assert!(result.is_ok());
                        assert_eq!(&operator, &expected_char);
                    }
                    Err(_) => {
                        assert!(result.is_err());
                    }
                }
            }
        };
    }

    create_test!(test_new_plus(Operator::default(), '+') = Ok(Operator::Add));
    create_test!(test_new_multiply(Operator::default(), '*') = Ok(Operator::Multiply));
    create_test!(test_new_space(Operator::default(), ' ') = Ok(Operator::Undefined));
    create_test!(test_existing_operator(Operator::Add, '*') = Err(anyhow::Error::msg("")));
    create_test!(test_invalid_char(Operator::default(), 'x') = Err(anyhow::Error::msg("")));
}
