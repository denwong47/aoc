//! A model for order-independent accumulative hashing of sets.
//!
//! This provides a default implementation of accumulative hashing tailored for sets
//! of unsigned integers, which is useful for scenarios like tracking visited nodes
//! in a DFS.

mod model;
pub use model::AccumulativeHashSet;
