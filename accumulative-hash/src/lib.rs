//! # Accumulative Hash / Additive Commutative Hash
//! 
//! `accumulative_hash` provides an efficient, order-independent hashing mechanism ideal for
//! tracking the state of additive data structures, such as paths in a Depth-First Search (DFS)
//! where the order of nodes visited does not change the identity of the path's *set* of nodes.
//! 
//! This library implements **Commutative Hashing** using modular arithmetic (`wrapping_add`/`wrapping_sub`),
//! guaranteeing that the order of insertion does not affect the final hash state.
//! 
//! ```text
//! H({A, B, C}) = H({A, C, B}) = H({C, B, A})
//! ```
//! 
//! This is achieved by pre-mixing each input value (`u32`, `u64`, etc.) into a high-quality, large
//! pseudorandom number, and then combining these numbers via modular addition.
//! 
//! ## Features
//! 
//! * **Order-Independent Hashing (Commutative):** No need to sort input elements ([`Vec<u32>`]) before hashing, saving ``O(N \log N)`` per check.
//! * **Incremental Updates:** ``O(1)`` addition (`add`) and removal (`remove`) of elements.``
//! * **Composition Property:** Hashes are associative, meaning two accumulated hashes can be summed to get the hash of the combined set of elements.
//! * **Thread Safety:** The [`AtomicAccumulativeHash`] struct uses [`std::sync::atomic::AtomicU64`], etc., with Compare-And-Swap (CAS) loops for lock-free, thread-safe updates to the hash state.
//!   * **NOTE**: This requires the ``atomic`` feature flag to be enabled.
//! * **Collision Resistance:** By default, uses large integer types (recommended [`u64`] and [`u128`]) and carefully chosen mixing constants derived from mathematical principles (like the Golden Ratio constant `0x9E3779B97F4A7C15F39CC0605CEDC834` for `u128`) to ensure a statistically low collision rate.
//! 
//! ## Example: DFS Path Tracking
//! 
//! This library solves the problem of tracking visited, order-independent paths in a DFS without expensive memory allocations or sorting.
//! 
//! ```rust
//! use accumulative_hash::AccumulativeHash;
//! use std::collections::HashSet; // other hash set implementations can be used as well
//! 
//! // Assuming your step IDs are u32
//! type PathHash = u64;
//! 
//! fn check_and_mark_path(
//!     visited: &mut HashSet<PathHash>,
//!     current_path_hash: PathHash,
//! ) -> bool {
//!     // True if we've visited this state before, regardless of the order of steps.
//!     !visited.insert(current_path_hash)
//! }
//! 
//! fn depth_first_search(steps: &[u32]) {
//!     let mut visited_states = HashSet::<PathHash>::default();
//!     let mut current_hash_state = AccumulativeHash::<PathHash>::new();
//! 
//!     for &step_id in steps {
//!         // 1. Add the step: O(1)
//!         current_hash_state.add(step_id);
//! 
//!         let path_hash = *current_hash_state.state();
//! 
//!         // 2. Check for visit: O(1) average
//!         if check_and_mark_path(&mut visited_states, path_hash) {
//!             println!("Path leading to {:X} was already visited!", path_hash);
//!             // Backtrack logic would follow here...
//!         }
//!         
//!         // ... continue DFS ...
//! 
//!         // 3. Backtrack (Remove step): O(1)
//!         // If we were using a recursive DFS, this would happen on return:
//!         // current_hash_state.remove(step_id);
//!     }
//! }
//! ```
//! 
//! ## Available Hash Types
//! 
//! The core trait for underlying types is [IsAccumulativeHashType], implemented for:
//! 
//! * [u8], [u16], [u32] (Not recommended for production due to high collision risk)
//! * [u64] (Good balance of speed and collision resistance)
//! * [u128] (Highest collision resistance)
//! * [usize] (Based on the target platform's pointer width)
//! 
//! For thread-safe operations, use the [`AtomicAccumulativeHash`] struct with the ``atomic`` feature enabled.
//! The underlying atomic types supported are:
//! 
//! * [AtomicU8] (Not recommended for production due to high collision risk)
//! * [AtomicU16] (Not recommended for production due to high collision risk)
//! * [AtomicU32] (Not recommended for production due to high collision risk)
//! * [AtomicU64] (Recommended for thread-safe operations with good collision resistance)
//! * [AtomicUsize] (Based on the target platform's pointer width)
//! 
//! [AtomicU8]: std::sync::atomic::AtomicU8
//! [AtomicU16]: std::sync::atomic::AtomicU16
//! [AtomicU32]: std::sync::atomic::AtomicU32
//! [AtomicU64]: std::sync::atomic::AtomicU64
//! [AtomicUsize]: std::sync::atomic::AtomicUsize

pub(crate) mod helpers;

mod traits;
pub use traits::*;

mod models;
pub use models::*;
