use crate::{SimpleGraphError, traits};
use num_traits::Zero;
use std::{cmp::Ord, fmt::Debug, hash::Hash};

pub struct NodeInProgress<'s, K, D, N> {
    node: &'s N,
    path_to_node: Vec<&'s K>,
    distance: D,
    popped_count: u8,
    unvisited_neighbours: Vec<(&'s N, D)>,
}

impl<'s, K, D, N> std::fmt::Debug for NodeInProgress<'s, K, D, N>
where
    K: Debug,
    D: Debug,
    N: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeInProgress {{ node: ")
            .and_then(|_| std::fmt::Debug::fmt(self.node, f))
            .and_then(|_| write!(f, ", path_to_node: [{:?}], ", self.path_to_node))
            .and_then(|_| write!(f, "distance: {:?}, ", self.distance))
            .and_then(|_| write!(f, "popped_count: {} }}", self.popped_count))
    }
}

impl<'s, K, D, N> NodeInProgress<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNode<'s, K, D>,
{
    pub fn new(
        node: &'s N,
        path_to_last_node: Option<&[&'s K]>,
        distance: D,
        get_node_by_key: impl Fn(&K) -> Option<&'s N>,
    ) -> Self {
        let path_to_node = {
            let mut path_to_node = path_to_last_node
                .map(|slice| slice.to_vec())
                .unwrap_or_default();
            path_to_node.push(node.id());
            path_to_node
        };

        Self {
            node,
            path_to_node,
            distance: distance,
            popped_count: 0,
            unvisited_neighbours: node.neighbours(get_node_by_key).collect::<Vec<_>>(),
        }
    }

    pub fn into_solution(self) -> (Vec<&'s K>, D) {
        (self.path_to_node, self.distance)
    }

    pub fn next_unvisited_neighbour(
        &mut self,
        get_node_by_key: impl Fn(&K) -> Option<&'s N>,
    ) -> Option<Self> {
        self.unvisited_neighbours.pop().map(|(node, distance)| {
            Self::new(
                node,
                Some(&self.path_to_node),
                self.distance.clone() + distance,
                get_node_by_key,
            )
        })
    }
}

pub struct Dfs<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNode<'s, K, D>,
{
    start: &'s N,
    destination: &'s N,
    memo: Vec<NodeInProgress<'s, K, D, N>>,
}

impl<'s, K, D, N> std::fmt::Debug for Dfs<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNode<'s, K, D>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dfs {{ start: ")
            .and_then(|_| std::fmt::Debug::fmt(self.start.id(), f))
            .and_then(|_| write!(f, ", destination: "))
            .and_then(|_| std::fmt::Debug::fmt(self.destination.id(), f))
            .and_then(|_| {
                write!(
                    f,
                    ", memo: {:?} }}",
                    self.memo
                        .iter()
                        .map(|n| n
                            .path_to_node
                            .iter()
                            .map(|k| format!("{:?}", k))
                            .collect::<Vec<_>>()
                            .join("->"))
                        .collect::<Vec<_>>()
                )
            })
    }
}

impl<'s, K, D, N> Dfs<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNode<'s, K, D>,
{
    pub fn new(
        start: &'s N,
        destination: &'s N,
        get_node_by_key: impl Fn(&K) -> Option<&'s N>,
    ) -> Result<Self, SimpleGraphError<K, D>> {
        if start.id() == destination.id() {
            return Err(SimpleGraphError::CannotPathToSelf {
                node: start.id().clone(),
            });
        }

        Ok(Self {
            start,
            destination,
            memo: vec![NodeInProgress::new(start, None, D::zero(), get_node_by_key)],
        })
    }

    pub fn next_solution(
        &mut self,
        get_node_by_key: impl Fn(&K) -> Option<&'s N> + Clone,
    ) -> Option<(Vec<&'s K>, D)> {
        loop {
            if self.memo.len() == 0 {
                break;
            }
            let opt_next_node = {
                self.memo
                    .last_mut()
                    .expect("Unreachable; memo length checked above")
                    .next_unvisited_neighbour(get_node_by_key.clone())
            };

            match opt_next_node {
                Some(next_node) => {
                    if next_node.node.id() == self.destination.id() {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Found solution at node {:?} with distance {:?} and path {:?}",
                            next_node.node.id(),
                            next_node.distance,
                            next_node.path_to_node
                        );
                        // Backtrack after yielding solution, because there should not multiple paths from A -> Destination
                        self.memo.pop();
                        return Some(next_node.into_solution());
                    } else {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Visiting node {:?} with distance {:?} and path {:?}",
                            next_node.node.id(),
                            next_node.distance,
                            next_node.path_to_node
                        );
                        self.memo.push(next_node);
                    }
                }
                None => {
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "Exhausted all neighbours for node {:?}, backtracking",
                        self.memo
                            .last()
                            .expect("Unreachable; memo length checked above")
                            .node
                            .id()
                    );
                    // Backtrack
                    self.memo.pop();
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests_dfs {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use crate::funcs::_tests::*;

    #[test]
    fn test_dfs() {
        let nodes: HashMap<u8, TestNode> = (1..=6).map(|id| (id, TestNode::new(id))).collect();

        let start_node = nodes.get(&1).expect("Start node not found");
        let destination_id = 5;
        let get_node_by_key = |key: &u8| nodes.get(key);
        let mut dfs = Dfs::new(start_node, get_node_by_key(&destination_id).expect("Unreachable, destination node not found"), get_node_by_key)
            .expect("Failed to create DFS instance");

        let solutions = {
            let mut sols = HashSet::new();
            while let Some(solution) = dfs.next_solution(get_node_by_key) {
                sols.insert((
                    solution.0.into_iter().map(|k| *k).collect::<Vec<u8>>(),
                    solution.1,
                ));
            }
            sols
        };

        let expected_solutions: HashSet<(Vec<u8>, u32)> = HashSet::from_iter([
            (vec![1, 6, 5], 23),
            (vec![1, 3, 4, 5], 26),
            (vec![1, 3, 6, 5], 20),
            (vec![1, 2, 4, 5], 28),
            (vec![1, 2, 3, 6, 5], 28),
            (vec![1, 2, 3, 4, 5], 34),
        ].into_iter());

        assert_eq!(solutions, expected_solutions);
    }
}
