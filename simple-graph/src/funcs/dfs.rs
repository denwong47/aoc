use crate::{SimpleGraphError, traits};
#[cfg(feature = "dfs-count")]
use fxhash::FxHashMap;
use num_traits::Zero;
use std::{cmp::Ord, fmt::Debug, hash::Hash};

pub struct NodeInProgress<'s, K, D, N> {
    node: &'s N,
    distance: D,
    next_index_to_visit: usize,
    _phantom: std::marker::PhantomData<K>,
}

impl<'s, K, D, N> std::fmt::Debug for NodeInProgress<'s, K, D, N>
where
    D: Debug,
    N: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeInProgress {{ node: ")
            .and_then(|_| std::fmt::Debug::fmt(self.node, f))
            .and_then(|_| write!(f, ", distance: {:?}, ", self.distance))
            .and_then(|_| write!(f, "next_index_to_visit: {} }}", self.next_index_to_visit))
    }
}

impl<'s, K, D, N> NodeInProgress<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNodeWithIndexedNeighbours<'s, K, D>,
{
    pub fn new(node: &'s N, distance: D) -> Self {
        Self {
            node,
            distance,
            next_index_to_visit: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn next_unvisited_neighbour(
        &mut self,
        get_node_by_key: impl Fn(&K) -> Option<&'s N>,
    ) -> Option<Self> {
        self.node
            .get_neighbour(self.next_index_to_visit, get_node_by_key)
            .map(|(node, distance)| {
                // Advance index for next call
                self.next_index_to_visit += 1;
                Self::new(node, self.distance.clone() + distance)
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
    tracker: Vec<NodeInProgress<'s, K, D, N>>,
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
                    ", tracker: {:?} }}",
                    self.tracker
                        .iter()
                        .map(|n| format!("{:?}", n.node.id()))
                        .collect::<Vec<_>>()
                        .join("->")
                )
            })
    }
}

impl<'s, K, D, N> Dfs<'s, K, D, N>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNodeWithIndexedNeighbours<'s, K, D>,
{
    pub fn new(
        start: &'s N,
        destination: &'s N,
        size_hint: usize,
    ) -> Result<Self, SimpleGraphError<K, D>> {
        if start.id() == destination.id() {
            return Err(SimpleGraphError::CannotPathToSelf {
                node: start.id().clone(),
            });
        }

        let mut tracker = Vec::with_capacity(size_hint);
        tracker.push(NodeInProgress::new(start, D::zero()));

        Ok(Self {
            start,
            destination,
            tracker,
        })
    }

    #[allow(unused_assignments)]
    pub fn next_solution(
        &mut self,
        get_node_by_key: impl Fn(&K) -> Option<&'s N> + Clone,
    ) -> Option<(Vec<&'s K>, D)> {
        while self.tracker.len() > 0 {
            let opt_next_node = {
                self.tracker
                    .last_mut()
                    .expect("Unreachable; memo length checked above")
                    .next_unvisited_neighbour(get_node_by_key.clone())
            };

            match opt_next_node {
                Some(next_node) => {
                    if next_node.node.id() == self.destination.id() {
                        let path_to_node = self
                            .tracker
                            .iter()
                            .chain(std::iter::once(&next_node))
                            .map(|n| n.node.id())
                            .collect::<Vec<&'s K>>();

                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Found solution at node {:?} with distance {:?} and path {:?}",
                            next_node.node.id(),
                            next_node.distance,
                            path_to_node
                        );

                        return Some((path_to_node, next_node.distance));
                    } else {
                        #[cfg(feature = "trace")]
                        {
                            let path_to_node = self
                                .tracker
                                .iter()
                                .chain(std::iter::once(&next_node))
                                .map(|n| n.node.id())
                                .collect::<Vec<&'s K>>();
                            eprintln!(
                                "Visiting node {:?} with distance {:?} and path {:?}",
                                next_node.node.id(),
                                next_node.distance,
                                path_to_node,
                            );
                        }
                        self.tracker.push(next_node);
                    }
                }
                None => {
                    // Backtrack
                    self.tracker.pop();
                }
            }
        }

        None
    }
}

#[cfg(feature = "dfs-count")]
pub fn dfs_count<'s, K, D, N>(
    start: &'s N,
    destination_id: &'s K,
    size_hint: usize,
    get_node_by_key: impl Fn(&K) -> Option<&'s N> + Clone,
) -> usize
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNodeWithIndexedNeighbours<'s, K, D>,
{
    let mut count = 0;
    let mut memoized_counts_by_node: fxhash::FxHashMap<&'s K, usize> =
        FxHashMap::with_capacity_and_hasher(size_hint, Default::default());
    let mut tracker: Vec<NodeInProgress<'s, K, D, N>> = Vec::with_capacity(size_hint);
    tracker.push(NodeInProgress::new(start, D::zero()));

    while tracker.len() > 0 {
        let opt_next_node = {
            tracker
                .last_mut()
                .expect("Unreachable; memo length checked above")
                .next_unvisited_neighbour(get_node_by_key.clone())
        };

        match opt_next_node {
            Some(next_node) if destination_id == next_node.node.id() => {
                // We found a solution
                count += 1;

                #[cfg(feature = "trace")]
                {
                    let path_to_node = tracker
                        .iter()
                        .chain(std::iter::once(&next_node))
                        .map(|n| n.node.id())
                        .collect::<Vec<&'s K>>();

                    eprintln!(
                        "Found solution at node {:?} with distance {:?} and path {:?}",
                        next_node.node.id(),
                        next_node.distance,
                        path_to_node
                    );
                }

                let last_node = tracker
                    .last()
                    .expect("Unreachable; memo length checked above");

                // If we have just reached a destination, the count increase is always 1 for the last node
                memoized_counts_by_node
                    .entry(last_node.node.id())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
            Some(next_node) if memoized_counts_by_node.contains_key(next_node.node.id()) => {
                // We have already computed the number of paths from this node to the destination
                let unique_paths_from_next_node = *memoized_counts_by_node
                    .get(next_node.node.id())
                    .expect("Unreachable; checked above");

                #[cfg(feature = "trace")]
                {
                    let path_to_node = tracker
                        .iter()
                        .chain(std::iter::once(&next_node))
                        .map(|n| n.node.id())
                        .collect::<Vec<&'s K>>();

                    eprintln!(
                        "Using memoized count {:?} at node {:?} with distance {:?} and path {:?}",
                        unique_paths_from_next_node,
                        next_node.node.id(),
                        next_node.distance,
                        path_to_node
                    );
                }

                count += unique_paths_from_next_node;

                let last_node = tracker
                    .last()
                    .expect("Unreachable; memo length checked above");
                memoized_counts_by_node
                    .entry(last_node.node.id())
                    .and_modify(|c| *c += unique_paths_from_next_node)
                    .or_insert(unique_paths_from_next_node);
            }
            Some(next_node) => {
                #[cfg(feature = "trace")]
                {
                    let path_to_node = tracker
                        .iter()
                        .chain(std::iter::once(&next_node))
                        .map(|n| n.node.id())
                        .collect::<Vec<&'s K>>();

                    eprintln!(
                        "Visiting node {:?} with distance {:?} and path {:?}, no memoized count found",
                        next_node.node.id(),
                        next_node.distance,
                        path_to_node,
                    );
                }
                tracker.push(next_node);
            }
            None => {
                // Backtrack
                let _popped = tracker.pop().expect("Unreachable; memo length checked above");
                
                #[cfg(feature = "trace")]
                {
                    let path_to_node = tracker
                    .iter()
                    .map(|n| n.node.id())
                    .collect::<Vec<&'s K>>();
                    eprintln!("Backtracking from node {:?} to path {:?}", _popped.node.id(), path_to_node);
                }
            
                if tracker.len() > 0 {
                    // We should update the memoization for the last node in the tracker, even if the count is zero
                    let count_from_popped = memoized_counts_by_node.get(&_popped.node.id()).copied().unwrap_or_default();
                    let last_node = tracker
                        .last()
                        .expect("Unreachable; memo length checked above");
                    memoized_counts_by_node
                        .entry(last_node.node.id())
                        .and_modify(|c| *c += count_from_popped)
                        .or_insert(count_from_popped);
                }
            }
        }
    }

    #[cfg(feature = "trace")]
    eprintln!("Final memoized counts: {:?}", memoized_counts_by_node);

    count
}

#[cfg(test)]
mod tests_dfs {
    use super::*;
    use crate::funcs::_tests::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_dfs() {
        let nodes: HashMap<u8, TestNode> = (1..=6)
            .map(|id| (id, TestNode::new_with_connections(id, CONNECTIONS)))
            .collect();

        let start_node = nodes.get(&1).expect("Start node not found");
        let destination_id = 5;
        let get_node_by_key = |key: &u8| nodes.get(key);
        let mut dfs = Dfs::new(
            start_node,
            get_node_by_key(&destination_id).expect("Unreachable, destination node not found"),
            nodes.len(),
        )
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

        let expected_solutions: HashSet<(Vec<u8>, u32)> = HashSet::from_iter(
            [
                (vec![1, 6, 5], 23),
                (vec![1, 3, 4, 5], 26),
                (vec![1, 3, 6, 5], 20),
                (vec![1, 2, 4, 5], 28),
                (vec![1, 2, 3, 6, 5], 28),
                (vec![1, 2, 3, 4, 5], 34),
            ]
            .into_iter(),
        );

        assert_eq!(solutions, expected_solutions);
    }

    #[test]
    #[cfg(feature = "dfs-count")]
    fn test_dfs_count() {
        let nodes: HashMap<u8, TestNode> = (1..=6)
            .map(|id| (id, TestNode::new_with_connections(id, CONNECTIONS)))
            .collect();

        let start_node = nodes.get(&1).expect("Start node not found");
        let destination_id = 5;
        let get_node_by_key = |key: &u8| nodes.get(key);
        let count = dfs_count(start_node, &destination_id, nodes.len(), get_node_by_key);
        assert_eq!(count, 6);
    }

    /// This test case emphasizes the single memoization aspect of DFS count
    #[cfg(feature = "dfs-count")]
    mod case_1 {
        use super::*;

        pub const CONNECTIONS: &[(u8, u8, u32)] = &[
            (1, 2, 1),
            (1, 3, 2),
            (1, 4, 3),
            (1, 5, 1),
            (4, 5, 4),
            (4, 6, 4),
            (4, 7, 4),
            (5, 7, 5),
            (6, 7, 6),
            (6, 10, 6),
            (7, 10, 7),
            (8, 10, 8),
            (9, 10, 9),
        ];

        #[test]
        fn test() {
            let nodes: HashMap<u8, TestNode> = (1..=10)
                .map(|id| (id, TestNode::new_with_connections(id, CONNECTIONS)))
                .collect();

            let start_node = nodes.get(&1).expect("Start node not found");
            let destination_id = 10;
            let get_node_by_key = |key: &u8| nodes.get(key);
            let count = dfs_count(start_node, &destination_id, nodes.len(), get_node_by_key);
            assert_eq!(count, 5);
        }
    }

    mod case_2 {
        use super::*;

        pub const CONNECTIONS: &[(u8, u8, u32)] = &[
            (1,2,1),
            (1,3,1),
            (2,3,1),
            (2,4,1),
            (3,5,1),
            (2,5,2),
            (3,4,2),
            (5,4,1),
            (4,6,1),
            (5,6,1),
            (6,7,1),
            (6,8,1),
            (7,8,1),
            (7,9,1),
            (8,10,1),
            (7,10,2),
            (8,9,2),
            (10,9,1),
            (9,11,1),
            (10,11,1),
        ];

        #[test]
        fn test() {
            let nodes: HashMap<u8, TestNode> = (1..=11)
                .map(|id| (id, TestNode::new_with_connections(id, CONNECTIONS)))
                .collect();

            let start_node = nodes.get(&1).expect("Start node not found");
            let destination_id = 11;
            let get_node_by_key = |key: &u8| nodes.get(key);
            let count = dfs_count(start_node, &destination_id, nodes.len(), get_node_by_key);
            assert_eq!(count, 81);
        }
    }
}
