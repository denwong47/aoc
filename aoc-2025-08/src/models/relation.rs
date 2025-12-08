use super::NodeDistanceType;

/// A relation between two nodes, characterized by the distance between them; used for building BinaryHeaps.
#[derive(Debug, Clone)]
pub struct Relation {
    pub node_a: usize,
    pub node_b: usize,
    pub distance: NodeDistanceType,
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Relation {}

impl Ord for Relation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Relation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
