use num_traits::Zero;
use std::{cmp::Ord, fmt::Debug, hash::Hash};

/// A trait representing a node in a graph.
pub trait IsNode<'s, K, D>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
{
    /// Get the unique identifier for the node.
    fn id(&self) -> &K;

    /// Get an iterator over the neighbours of this node along with the associated data.
    fn neighbours(
        &'s self,
        get_node_by_key: impl Fn(&K) -> Option<&'s Self>,
    ) -> impl Iterator<Item = (&'s Self, D)>;
}
