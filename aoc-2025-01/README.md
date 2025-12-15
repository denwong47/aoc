# Day 1

Not much to talk about here, we use `Wheel` to store the current state, then we `Wheel::rotate` it based on the input.

Assuming the wheel has `S` segment (where in the example, `S=100`), current state is `X` and the input is `LN` or `RN`.
Then we derive `P = (N+X) % S` and `R = (N+X) // S` (integer division).
- Rotate Right, any amount
    - Number of times we lapped `0` is `R`
    - New position is `P`
- Rotate Left, `P > 0`
    - Number of times we lapped `0` is `0` - since we are still positive, we didn't cross `0`
    - New position is `P`
- Rotate Left, `P <= 0` , and `X > 0`
    - i.e. We have rotated from some positive position back to `0`
    - Number of times we lapped `0` is `R + 1`, since the only reason why we are at negative is because we crossed `0` from a positive `X`
    - New position is `P + S` (to convert negative position to positive)
- Rotate Left, `P <= 0`, and `X == 0`
    - i.e. We have rotated from `0` back to `0`
    - Number of times we lapped `0` is `R`, since we started at `0` we don't count it again
    - New position is `P + S` (to convert negative position to positive)

The implementation gives `Wheel` a generic constant parameter `S` to define the size of the wheel, defaulting to `100` if not specified. This allows for flexibility in case the problem requirements change or if different wheel sizes are needed for testing or other purposes. However this does not allow dynamic sizing of the wheel at runtime, as `S` needs to be known at compile time.