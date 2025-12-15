# Day 3 - Lobby

There is only one logic here:

- For each line, look at a sliding window of size 2 across the digits, say `AB`.
  - if `A >= B`, then we advance the window by 1.
  - if `A < B`,
    - if this is the start of line, discard `A` and continue evaluating the same window;
    - otherwise, discard `A` and backtrack one character to the left, so we can evaluate `B` against the new left character in the next iteration.
  - if we then reach the end of the line, or we only have `N` characters left, we keep the first `N` characters and return.

## Implementation Details

Since our main data structure is just a sequence of digits, we can represent it as a `VecDeque<u8>`. To simplify the calling code, we can encapsulate the logic into a method, and add it to any `struct` we wanted via a `trait`.
This prompted the `trait` `HighestSequentialCombination` with a method `filter_to_highest_sequential_combination<const N: usize>(&mut self)`, and we implemented it for `VecDeque<u8>`. In hindsight, `Vec<u8>` would have sufficed since we only need to backtrack by one character, but the actual implementation does not make much difference.