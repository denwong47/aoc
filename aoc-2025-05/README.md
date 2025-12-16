# Day 5

`RangeInclusive` merging and intersection, that's about it.

Except I didn't really read the input before diving in, so I didn't realize the bounds of the ranges are well within `u64`; so I invented `StringRange` which allows ranges over arbitrary strings, so long as the ranges of each are within `u64` bounds. This could be easily modified to `u128` or whatever, but it was a complete overkill anyway.

Due to this self-imposed complexity, there are some inefficiencies involved mainly due to excessive string padding. There could be better ways to handle strings of different lengths, but I did not spend more time on it.
