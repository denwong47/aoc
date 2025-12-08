//! Simple functional implementations of common graph algorithms,
//! such as Dijkstra's.
//!
//! Skipping any concrete data structures, this crate focuses on providing
//! traits and algorithms that can be implemented on top of any graph
//! representation.

mod errors;
pub mod traits;
pub mod wrapper;
pub use errors::*;

mod funcs;
pub use funcs::*;
