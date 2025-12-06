/// Trait definitions for adding characters to different buffer types.
pub trait AddToBuffer: Default {
    fn add_to_buffer(&mut self, input: char) -> anyhow::Result<char>;
}

impl AddToBuffer for u16 {
    /// Part 1: Each character is added as a digit to the u16 buffer
    /// as a base 10 number.
    fn add_to_buffer(&mut self, input: char) -> anyhow::Result<char> {
        if input == ' ' {
            // Space indicates end of number input
            return Ok(input);
        } else if !input.is_ascii_digit() {
            anyhow::bail!("Invalid digit character: {:?}", input);
        }

        let digit = input.to_digit(10).unwrap() as u16;
        *self = self
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .ok_or_else(|| anyhow::anyhow!("Overflow when adding digit {:?} to buffer", input))?;

        Ok(input)
    }
}

impl AddToBuffer for Vec<Option<u8>> {
    /// Part 2: Each character is added as an Option<u8> to the buffer.
    ///
    /// ' ' (space) is represented as None, digits are Some(digit).
    ///
    /// If the position of the character is important, whitespace can be represented as
    /// `Some(0)` if needed.
    fn add_to_buffer(&mut self, input: char) -> anyhow::Result<char> {
        match input {
            ' ' => self.push(None),
            d if d.is_numeric() => self.push(Some(d as u8 - b'0')),
            _ => anyhow::bail!("Invalid character for digit buffer: {:?}", input),
        }
        Ok(input)
    }
}

#[cfg(test)]
mod test_add_u16 {
    use crate::AddToBuffer;

    macro_rules! create_test {
        ($name:ident($initial:expr, $char:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let mut buffer: u16 = $initial;
                let result = buffer.add_to_buffer($char);

                let expected: anyhow::Result<u16> = $expected;
                match expected {
                    Ok(expected_value) => {
                        assert!(result.is_ok());
                        assert_eq!(buffer, expected_value);
                    }
                    Err(_) => {
                        assert!(result.is_err());
                    }
                }
            }
        };
    }

    create_test!(add_valid_digit_0(0, '0') = Ok(0));
    create_test!(add_valid_digit_5(0, '5') = Ok(5));
    create_test!(add_valid_digit_9(0, '9') = Ok(9));
    create_test!(add_0_to_existing_value(12, '0') = Ok(120));
    create_test!(add_digit_to_existing_value(12, '3') = Ok(123));
    create_test!(add_invalid_character(0, 'a') = Err(anyhow::anyhow!("")));
    create_test!(add_digit_causing_overflow(6553, '6') = Err(anyhow::anyhow!("")));
}
