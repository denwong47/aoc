use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimpleGraphError<K: std::fmt::Debug, D: std::fmt::Debug> {
    #[error("node {destination:?} not connected to {start:?}")]
    NodeNotConnected { start: K, destination: K },

    #[error("cannot attempt to path from {node:?} to itself")]
    CannotPathToSelf { node: K },

    #[error("distance from {start:?} to {destination:?} has negative distance {distance:?}")]
    NegativeDistance {
        start: K,
        destination: K,
        distance: D,
    },

    #[error("this should be unreachable: {0}")]
    Unreachable(String),

    #[error("unknown simple graph error")]
    Unknown,
}
