# Day 10

The unsolvable.

## The thought

We were treating this as a pathing problem. Take this line as an example:

```
[...#] (0,1,2) (1,2,3) (1,2) (3) (0,2,3) {19,14,32,51}
```

We would then treat this as a 4 dimension space, from `(0,0,0,0)` to `(19,14,32,51)`.

We have 5 vectors at our disposal:
- `(1,1,1,0)` - which corresponds to `(0, 1, 2)`
- `(0,1,1,1)` - which corresponds to `(1, 2, 3)`
- `(0,1,1,0)` - which corresponds to `(1, 2)`
- `(0,0,0,1)` - which corresponds to `(3)`
- `(1,0,1,1)` - which corresponds to `(0, 2, 3)`

Our job is to find the shortest number of steps using these vectors to reach the target point.

Therefore we define our heuristic as the Euclidean distance from our current point to the target point (for simplicity, we can ignore the square root, as it doesn't change the ordering of distances), and we can perform a A* search to find the optimal path.

However as it turns out, some of the input are like `{247,209,47,244,247,253,271,268,251,238}` in magnitude, which makes the search space explode, and the A* search becomes very expensive.

This was also before the `accumulative-hash` crate was written (see separate crate), which would have made memoization a lot easier.

So this was a no go.

## Using a solver

Much of the internet was raving about Z3 or Coin-or CBC as solvers for constraint satisfaction problems. Which it absolutely is - we threw the problem at `good_lp` crate, using CoinCbc as the backend solver, and it solved all the problems in a matter of milliseconds. Which is an absolutely hollow victory without any sense of accomplishment. One does have to marvel at the power of these solvers though, and its good to have some awareness of what they can do, and may be we can use them in the future for practical problems.

To run with CoinCbc, install it first (on macOS), and then run with the `milp` feature pointed to the library path:

```sh
brew install cbc
RUSTFLAGS='-L /opt/homebrew/lib' cargo run --release --features=milp
```

On Linux the Library paths should be set up automatically if installed via package managers.

## Reddit solution

There is a neat solution we have yet to try:

- For any given requirement of `{W, X, Y, Z}` with the solution of `S`, we can be certain that the solution for `{kW, kX, kY, kZ}` is `kS` for any positive integer `k`, as long as all the vectors are binary in length in each dimension.
- Therefore, we can keep solve each problem by repeating two stages:
  - Find the minimum solution so that the requirements are reduced to all even numbers.
  - Bisect the requirements by 2
  - Repeat until all requirements are zero.
- The total solution is the sum of all the minimum solutions multiplied by the respective powers of 2.

This is a neat solution, and we will try to implement it when we have time.