use std::str::Chars;

use super::AddToBuffer;

pub struct BufferedLineReader<'s, T: AddToBuffer> {
    chars: Chars<'s>,
    pub buffer: T,
}

impl<'s, T: AddToBuffer> BufferedLineReader<'s, T> {
    /// Creates a new BufferedLineReader from the given line.
    ///
    /// The input string needs to remain in scope for the lifetime of the reader.
    pub fn new(line: &'s str) -> Self {
        Self {
            chars: line.chars(),
            buffer: T::default(),
        }
    }

    /// Advances the reader by one character, adding it to the buffer.
    pub fn advance(&mut self) -> anyhow::Result<Option<char>> {
        if let Some(ch) = self.chars.next() {
            self.buffer.add_to_buffer(ch)?;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }

    /// Yields the current buffer and resets it to default.
    pub fn yield_buffer(&mut self) -> T {
        let mut new_buffer = T::default();
        std::mem::swap(&mut self.buffer, &mut new_buffer);

        new_buffer
    }
}

impl<'s, T: AddToBuffer> Iterator for BufferedLineReader<'s, T> {
    type Item = anyhow::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.advance() {
            Ok(Some(ch)) => Some(Ok(ch)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod test_reader {
    use super::*;
    use crate::Operator;

    macro_rules! create_test {
        // We have to make this `ident` for the <T> to work properly;
        // If we made it a `ty`, it would think the whole thing is a type.
        ($name:ident::<$type:ident>($line:literal) = $expected:expr) => {
            #[test]
            fn $name() {
                let line = $line;
                let mut reader = BufferedLineReader::<$type>::new(line);

                while let Some(result) = reader.next() {
                    result.expect("Failed to read character");
                }

                assert_eq!(reader.yield_buffer(), $expected);
                assert_eq!(&reader.buffer, &$type::default());
            }
        };
    }

    create_test!(test_simple::<u16>("123") = 123);
    create_test!(test_empty::<u16>("") = 0);
    create_test!(test_simple_op::<Operator>("+  ") = Operator::Add);
    create_test!(test_empty_op::<Operator>("") = Operator::default());
    create_test!(test_number_trailing_spaces::<u16>("12345       ") = 12345);
}
