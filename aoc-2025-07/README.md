# Day 7

It was fairly obvious what Part 2 is about from the get go, but I got all the terminologies wrong, so you will find the code talking about "Beam Intensity" which is what the actual problem considered "Timelines".

The main function `calculate_beam_intensity_map` takes a
- line of input, and
- optionally a `HashMap<usize, u64>` representing the beam intensities from the previous line.

Since we only cared about columns which had a beam intensity from the previous line, there is no need to read the whole input apart from the first line. We just look at the columns which had beam intensities in the previous line, match it with the corresponding character in the current line, and calculate the new beam intensity accordingly.

## Implementation Details

We are finally learning a lesson and starting using type aliases _just in case_ we want to change the underlying type later.
