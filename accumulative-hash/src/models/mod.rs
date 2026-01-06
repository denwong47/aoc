//! Models for accumulative hashing.

mod standard;
pub use standard::AccumulativeHash;

#[cfg(feature = "atomic")]
mod atomic;
#[cfg(feature = "atomic")]
pub use atomic::AtomicAccumulativeHash;

#[cfg(feature = "set")]
mod set;
#[cfg(feature = "set")]
pub use set::*;