# Day 11

Memoization, memoization, memoization.

Since we are finding _every single unique path_ to reach the target, there are no algorithmic optimizations we can do to prune the search space, as every path counts. However memoization can help us avoid re-computing the same sub-problems multiple times.

This can be done exceptionally easily in Python with `functools.lru_cache`, but Rust doesn't like global state especially mutable ones, so we will have to do it ourselves with a `HashMap` mapping from the ID of visited node to the number of unique paths from that node to the target. `0` is a valid number of paths, so the same cache can be used for both memoization hits and misses.

> [!TIP]
> The good thing is pathing sequence actually matters here, so the hash can be of the `u32` ID of the node only. If the problem was additive, we should have used `AccumulativeHash` instead.

The algorithm is just a standard DFS with memoization:

- DFS until we are at `source_node`, looking at all its neighbors `next_node` where
    - `next_node` is the target, then `c = 1`, or
    - `next_node` is a memoization hit, then `c = memo[next_node_id]`, or
    - there are no `next_node` then `c = 0` (which the input data does not have)
- We then set `memo[source_node_id] += c`. Don't backtrack yet, in case there are more neighbors that are like `source -> target` and `source -> intermediary -> target`.
- If there are no more neighbours, backtrack.
    - if then the depth tracker has no more nodes, return `memo[source_node_id]`.
    - otherwise we pop the last node from the depth tracker, then set `memo[layer_n_id] += memo[layer_n+1_id]`.

      > [!TIP]
      > This number is not final until we have explored all neighbors of `layer_n_id`, so we do not return yet.

## Why not recursion?

Recursion is possible, but Rust has a very low default stack size (2MB on Windows) which can be easily exceeded with deep recursion. While we can increase the stack size, it is not a good idea to rely on that; we should always prefer iterative solutions in Rust.

This makes the `loop {}` a bit more verbose than a simple recursive function, but it is safer when the depth count is unknown.

## Lessons

```text
DFS count completed in 367 iterations
Part 1: Total number of distinct paths: 796
Part 1 completed in: 77.25µs
DFS count completed in 2340 iterations
Number of paths from SVR to DAC: 1040248093572
DFS count completed in 358 iterations
Number of paths from SVR to FFT: 5418
DFS count completed in 363 iterations
Number of paths from DAC to FFT: 0
DFS count completed in 1471 iterations
Number of paths from FFT to DAC: 13733136
DFS count completed in 437 iterations
Number of paths from DAC to OUT: 3952
DFS count completed in 1642 iterations
Number of paths from FFT to OUT: 3822779890610
Part 2: Total number of valid paths: 294053029111296
Part 2 completed in: 518.5µs
```

Memoization is a powerful technique to optimize recursive or DFS algorithms that have overlapping subproblems. By storing the results of expensive function calls and reusing them when the same inputs occur again, we can significantly reduce actual computation, despite the re-allocation overhead in resizing the `HashMap`. Supposingly, we know the exact number of unique nodes, we can even pre-allocate the `HashMap` capacity to avoid resizing altogether.

A previous version of this solution used the generic `dfs` implementation from the `simple-graph` crate (also in this repo), which is a mutable struct that yields solutions via an iterator interface. While it works, there are two huge overheads:

- now we can't just sum the cache together, as we need to yield each unique path count per source node; so we have to store the entire path count per source node in a `Vec<Vec<u32>>`, which is a lot of allocations.
- by having to return a path (i.e. `Vec<u32>`) per iteration, we are allocating a lot of memory for each path, which is then dropped immediately after use, which is wasteful.

These alone made the previous solution impossible to finish in reasonable time, even with memoization.

**While generic solutions are nice to have, sometimes a problem calls for a specialized solution, and you can't be lazy about it.**
