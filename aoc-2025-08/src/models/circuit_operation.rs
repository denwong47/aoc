#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitOperation {
    Join {
        node_a: usize,
        node_b: usize,
        updated: usize,
    },
    NoOp {
        node_a: usize,
        node_b: usize,
    },
}
