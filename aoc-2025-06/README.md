# Day 6

By using one buffer per line in the input, we read one character at a time.

- if

  - all the characters in the current position across all buffers are whitespace, or
  - all buffers have reached EOF,

  then we consider the current segment to be complete, and we process it.

- otherwise, we ask each buffer to accumulate characters in their `AddToBuffer` implementor.

There are 3 implementations for `AddToBuffer`:

- `Operator` - used for the last line only, which stores the operator character (`+` or `*`) for the segment. If a second operator is encountered before processing the segment, an error is raised.
- `u16` for Part 1 - each buffer will accumulate the characters horizontally, treating each sucessive character as a digit in a base 10 number.
- `Vec<Option<u8>>` for Part 2 - each buffer will accumulate the digits separately in a vector. If a whitespace character is encountered, it is stored as `None` in the vector. This preserves the position of each digit for later processing, which requires each of these buffers to be zipped vertically to get the number in base 10 for each position again.
