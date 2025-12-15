# Day 2

We know a few things:
- `(value as f32).log10().floor() + 1` gives number of digits in `value`, let's call that `E`.
- the patterns that we are after is divisible by numbers in the pattern of `101`, `1001`, `10101` up to `E` digits.

For each integer from `1` to `E // 2`, we can generate a mask pattern by:

```rust
fn generate_mask(pattern_length: usize, repeats: usize) -> u64 {
    (0..repeats).fold(0u64, |acc, i| acc + 10u64.pow((i * pattern_length) as u32))
}
```

The search itself did not employ any tricks, just brute force checking each number in the range to see if it is divisible by any of the generated masks.

One can optimize further by iterating the numbers in steps of all the masks, but that was not necessary to get the answer in a reasonable time.