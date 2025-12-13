# Accumulative Hash / Additive Commutative Hash

`accumulative_hash` provides an efficient, order-independent hashing mechanism ideal for
tracking the state of additive data structures, such as paths in a Depth-First Search (DFS)
where the order of nodes visited does not change the identity of the path's *set* of nodes.

This library implements **Commutative Hashing** using modular arithmetic (`wrapping_add`/`wrapping_sub`),
guaranteeing that the order of insertion does not affect the final hash state.

$$H(\{A, B, C\}) = H(\{A, C, B\}) = H(\{C, B, A\})$$

This is achieved by pre-mixing each input value (`u32`, `u64`, etc.) into a high-quality, large
pseudorandom number, and then combining these numbers via modular addition.

## Features

* **Order-Independent Hashing (Commutative):** No need to sort input elements (`Vec<u32>`) before hashing, saving $O(N \log N)$ per check.
* **Incremental Updates:** $O(1)$ addition (`add`) and removal (`remove`) of elements.
* **Composition Property:** Hashes are associative, meaning two accumulated hashes can be summed to get the hash of the combined set of elements.
* **Thread Safety:** The `AtomicAccumulativeHash` struct uses [`std::sync::atomic::AtomicU64`], [`AtomicU128`], etc., with Compare-And-Swap (CAS) loops for lock-free, thread-safe updates to the hash state.
* **Collision Resistance:** By default, uses large integer types (recommended [`u64`] and [`u128`]) and carefully chosen mixing constants derived from mathematical principles (like the Golden Ratio constant `0x9E3779B97F4A7C15F39CC0605CEDC834` for `u128`) to ensure a statistically low collision rate.