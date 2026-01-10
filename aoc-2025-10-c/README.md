# Day 10 in C

Re-writing Day 10 in C, wrote the nicest C code I had so far, and failed Part 2 again with a new approach.

## Part 1 Methodology

The buttons in this part are a toggle - which means that any 2nd press of the same button will undo the 1st. This asserts that *the solution must not contain any repeated button presses*.

This hugely simplifies the problem. Given there are no more than 12 buttons, it is trivial to just loop through all the combinations from 1 button to N buttons, then return on the first found solution.

We can introduce heuristics to educate the BFS a bit better, but for the most part this is good enough so that we can put on time into Part 2.

> [!TIP]
> The only interesting part is that we need a Generator struct for these combinations, as `Itertools::combinations` do not exist in C. Coming from Rust where `impl<'p, T> Iterator<item=&'p T> for MyGenerator<'p, T>` for a custom struct is a common pattern, this was not too hard, but it was good to have a go nonetheless.

## Part 2 - Attempt 1 Methodology (it doesn't work)

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

## Part 2 - Attempt 2 Methodology (you guessed it, doesn't work)

Despite all that, there is still clear value in the `2S => {2u,2v,2x,2y}` observation. The problem we encountered is for the parity, not the concept of division itself; so it is possible that we can look at the problem in a different way:

`{u,v,x,y} = {'u,'v,'x,'y} + {'u,'v,'x,'y} + ... + {'u,'v,'x,'y} + R`

Where none of `'u,'v,'x,'y` exceeds a certain constant of `C`, and both `{'u,'v,'x,'y}` and `{'u,'v,'x,'y} + R` have solutions.

It is hoped that in this approach, the parity of `{u,v,x,y}` will be preserved in `{'u,'v,'x,'y} + R`, and the solutions shall be the same. What we need to do however is to iterate on `C`, which requires 2 DFS per iteration to confirm if both sides have answers. However if we start with sufficiently small `C` (e.g. 5), then the DFS should not take long at all.

## Why it doesn't work V2

For the most part, this solution works a lot better than the previous one, but the parity problem remains unsolved to some degree.

Imagine a similar situation as above: we have a button for `(2,3)` and separate buttons for `(2)` and `(3)`, then some other buttons of `(1,3)` and `(1,2)`. Either solutions can choose to prefer `(2)` + `(1,3)` or `(3)` + `(1,2)`, which from their perpectives would do exactly the same thing; however:

- if they both choose the same, then the resultant solution would have a stack of `(2)` or `(3)` which would not have been optimisable; or
- if they choose differently, now we might have `n * (2) + n * (3)` which should have been merged into `n * (2,3)` instead.

When this situation does not exist, it solves solutions with few (<=5) buttons reasonably well, even if the vector lengths are 200+. For higher button counts, if it starts off from the wrong leg in DFS, the process would not return; this is down to DFS inefficiencies, which would take memoization + IDA* to improve. The current heuristics is showing its limits as well, as being greedy in euclidean distance does not always result in a solution.

## Part 2 - Attempt 3 Methodology (improvements in timing, but some solutions not optimised)

During Attempt 2, it was noticed that proving no solutions exists requires transversal of the full graph, which exponentially scale up to the number of buttons and/or the number of steps. The obvious solution is memoization, but that involves implementing

- Hash Set
- Additive Commutative Hash

in C, which is a lot of work in itself.

Since those two things already exists in Rust with the latter specifically written for Day 10 and 12, it seems logical to simply provide a CFFI to C, instead of replicating.

This was done, and now solving scenarios with 12 buttons and solutions like `{247,209,47,244,247,253,271,268,251,238}` takes only a second; but the above parity problem still exists, i.e some solutions will be suboptimal (294 presses vs 293 presses) The division strategy also would take some improvements, as certain certain chunking do not have solutions in either quotient or remainder, resulting in a fallback to DFS. While memoization does make it faster, it still exceeds the what is reasonable for a AOC challenge.

IDA* should be considered; it would help with the latter, but since the problem originated from the merging of solutions instead of the DFS step itself, IDA* will still not result in the optimal solution.

Since we have a solution, it is perhaps possible to add an "optimisation" step by descending sorting buttons by their effect counts, then BFS for each button (or button combinations) to look for combinations that can be optimised:

- `(2) + (3) => (2,3)`
- `(0,1) + (2,3) + (4,5) => (0,1,2) + (3,4,5)`
