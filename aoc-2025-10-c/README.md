# Day 10 in C

Re-writing Day 10 in C, wrote the nicest C code I had so far, and failed Part 2 again with a new approach.

## Part 1 Methodology

The buttons in this part are a toggle - which means that any 2nd press of the same button will undo the 1st. This asserts that *the solution must not contain any repeated button presses*.

This hugely simplifies the problem. Given there are no more than 12 buttons, it is trivial to just loop through all the combinations from 1 button to N buttons, then return on the first found solution.

We can introduce heuristics to educate the BFS a bit better, but for the most part this is good enough so that we can put on time into Part 2.

> [!TIP]
> The only interesting part is that we need a Generator struct for these combinations, as `Itertools::combinations` do not exist in C. Coming from Rust where `impl<'p, T> Iterator<item=&'p T> for MyGenerator<'p, T>` for a custom struct is a common pattern, this was not too hard, but it was good to have a go nonetheless.

## Part 2 Methodology (it doesn't work)

Since the problem was NP-hard, Reddit went straight to Z3 and CBC, which as we found in Rust, did solve the problem in incredible speed. However there was a post in the following day that intrigued me - it suggested using Part 1 to solve Part 2.

Take the first example:

`[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`

Since all buttons only advance each dimension by a maximum of `1`, this means that any solution for a given set:

`S => {u,v,x,y}`

We can simply double it to get the solution for double that vector:

`2S => {2u,2v,2x,2y}`

The new solution can only be invalid if there are some buttons that advanced some dimension by `>1`, which is not the case here.

That also means that we can supposingly split the problem into 2:

`{u,v,x,y} = {'u,'v,'x,'y} + R`

Where:
- `R` is whatever it needs so that
  - `'u,'v,'x,'y` are all divisible by 2
  - `R` has a solution on its own, found using the mechanism from Part 1.
  
Then we can bisect `{'u,'v,'x,'y}`, and recurse the above process, and multiply the solution by `2` in the end.

## Why it doesn't work

There are two reasons why this won't work:

- *Parity*: This approach assumes the parity for `R` is the same as the global solution, which is not true. In the above example, we have a button for `(2,3)` and separate buttons for `(2)` and `(3)`; it is possible that `R` calls for `(2)` on its own, and `{'u,'v,'x,'y}` requires `(3)`, but when combined as a global solution a single press of `(2,3)` would have been more efficient.
- *Unsolvable*: The above parity problem also makes some of `R` unsolvable. Since we have transformed the problem into different ones, it is possible that `R` never existed since no button presses could have arrived at something like `{1,0,0,0}` at all.

## Further Ideas

Despite all that, there is still clear value in the `2S => {2u,2v,2x,2y}` observation. The problem we encountered is for the parity, not the concept of division itself; so it is possible that we can look at the problem in a different way:

`{u,v,x,y} = {'u,'v,'x,'y} + {'u,'v,'x,'y} + ... + {'u,'v,'x,'y} + R`

Where none of `'u,'v,'x,'y` exceeds a certain constant of `C`, and both `{'u,'v,'x,'y}` and `{'u,'v,'x,'y} + R` have solutions.

It is hoped that in this approach, the parity of `{u,v,x,y}` will be preserved in `{'u,'v,'x,'y} + R`, and the solutions shall be the same. What we need to do however is to iterate on `C`, which requires 2 DFS per iteration to confirm if both sides have answers. However if we start with sufficiently small `C` (e.g. 5), then the DFS should not take long at all.
