use super::{Node, NodeCoordType, NodeDistanceType, Relation};
use kdtree::KdTree;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

/// A list of nodes that iterates over unique relations sorted by distance.
///
/// This struct owns the list of nodes as well as the KD-Tree used for efficient
/// nearest-neighbour computation; it can produce a [`ClosestNeighboursIterator`] that
/// yields unique relations between nodes in order of increasing distance.
///
/// ## Concept
///
/// Conceptually speaking, for any ``N`` number of nodes, there exists a complete matrix
/// of ``N x N`` distances between each node and every other node. If one is to take this
/// matrix,
///
/// - remove all self-referential distances (i.e. distance from node A to node A),
/// - remove all duplicate distances (i.e. A-B is the same as B-A), and then
/// - sort the remaining distances in ascending order,
///
/// then this list is what this struct iterates over:
///
/// ```text
///     p3 -> p6 = 2
///     p4 -> p5 = 6
///     p1 -> p2 = 7
///     p1 -> p3 = 9
///     p5 -> p6 = 9
/// ```
///
/// However computing this complete matrix is ``O(N^2)``, which is infeasible for large
/// ``N``.
///
/// If we are to look at the problem differently, instead of the full matrix that we sort,
/// we can have a sorted list of nearest-neighbour distances for each node:
///
/// ```text
///     p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
///     p2: p2 -> p1 = 7, p2 -> p3 = 10, ...
///     p3: p3 -> p6 = 2, p3 -> p1 = 9, ...
///     p4: p4 -> p5 = 6, p4 -> p3 = 11, ...
///     p5: p5 -> p4 = 6, p5 -> p6 = 9, ...
///     p6: p6 -> p3 = 2, p6 -> p5 = 9, ...
/// ```
///
/// Then we can scan the first nearest-neighbour of each node, the pop the smallest distance
/// from that list, shifting the next nearest-neighbour of that node to the front:
///
/// ```text
///     - popped p3 -> p6 = 2
///     - p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
///     - p2: p2 -> p1 = 7, p2 -> p3 = 10, ...
///     - p3: p3 -> p1 = 9, *p3 -> p2 = 10*, ...
///     - p4: p4 -> p5 = 6, p4 -> p3 = 11, ...
///     - p5: p5 -> p4 = 6, p5 -> p6 = 9, ...
///     - p6: p6 -> p3 = 2, p6 -> p5 = 9, ...
/// ```
///
/// There is no difference in the final sorted order of distances between this approach and the
/// complete matrix approach. However, this approach allows for lazy evaluation of distances,
/// and only requires the computation of nearest-neighbours for each node, when the node was
/// popped: in the example above, we may not even know about ``p3 -> p2 = 10`` at the time when
/// we pop ``p3 -> p6 = 2``, and we only compute it afterwards to fill the gap.
///
/// You may notice that the above examples have a lot of duplicate distances (e.g. ``p1 -> p2 = 7`` and
/// ``p2 -> p1 = 7``). This can be avoided by only asking each node to find its nearest-neighbours
/// where ``pN`` is higher than itself (i.e. only the bottom half of the distance matrix):
///
/// ```text
///     p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
///     p2: p2 -> p3 = 10, ...
///     p3: p3 -> p6 = 2, ...
///     p4: p4 -> p5 = 6, ...
///     p5: p5 -> p6 = 9, ...
/// ```
///
/// ``p6`` has no entries because it is the highest node. Any node that is only connected
/// by nodes lower than itself will not have any entries as well.
///
/// This ensures our whole table only has ``<N-1`` entries at any time, and they shall always
/// be non-repeating. We can be assured that this produces the same result, because if one of the
/// repeats was the smallest distance, its counterpart in the other direction would have been
/// in the heap at the same time, so keeping both is redundant.
///
/// ## Summary
///
/// To summarize, this struct, when using [`Self::iter_closest_neighbours`] (which
/// produces a [`ClosestNeighboursIterator`]), implements the following algorithm:
///
/// - builds a KD-Tree from the input nodes for efficient nearest-neighbour computation,
/// - for each node, finds its nearest-neighbour that has not already been paired with it
///  (i.e. only the bottom half of the distance matrix), and
/// - stores these relations in a min-heap sorted by distance,
/// - when popping a relation from the heap, fans out from the ``node_a`` of that relation
///   to find its next nearest-neighbour that has not already been paired with it, and
///   pushes that new relation onto the heap, replacing the popped relation.
/// - this continues until all unique relations have been popped from the heap, or
///   some stopping condition is met, e.g. all nodes have been joined into a single graph.
pub struct NodesList {
    pub nodes: Vec<Node>,
    pub tree: KdTree<NodeDistanceType, usize, Node>,
}

impl NodesList {
    /// Build a NodesList from a list of nodes.
    ///
    /// This will use a KD-Tree to efficiently compute nearest neighbors. Then for each
    /// node, it will find its nearest neighbours that had not already been paired with it,
    /// and store the resulting relations in a min-heap sorted by distance.
    ///
    /// This allows us to iterate over all unique nodes in order of increasing distance to
    /// nearest neighbour - which is useful in joining cluster of nodes into trees based
    /// on proximity.
    pub fn build_from(nodes: Vec<Node>) -> anyhow::Result<Self> {
        let mut tree = KdTree::new(3);

        nodes
            .iter()
            .enumerate()
            .try_for_each(|(i, node)| tree.add(*node, i))
            .map_err(|e| anyhow::anyhow!("Failed to build KD-Tree from nodes: {}", e))?;

        Ok(Self { nodes, tree })
    }

    /// Build a NodesList from a textual representation of nodes.
    pub fn build_from_text(input: &str) -> anyhow::Result<Self> {
        let nodes: Vec<Node> = input
            .lines()
            .map(|line| {
                let coords: Vec<NodeCoordType> = line
                    .split(',')
                    .map(|part| part.trim().parse::<NodeCoordType>())
                    .collect::<Result<_, _>>()
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to parse node coordinates from line '{}': {}",
                            line,
                            e
                        )
                    })?;

                if coords.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "Expected 3 coordinates per node, got {} in line '{}'",
                        coords.len(),
                        line
                    ));
                }

                Ok([coords[0], coords[1], coords[2]])
            })
            .collect::<Result<_, _>>()?;

        Self::build_from(nodes)
    }

    /// Get a reference to a node by its ID.
    pub fn get_node_by_id(&self, node_id: usize) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    /// Get an iterator over unique relations sorted by distance.
    pub fn iter_closest_neighbours<'a>(&'a self) -> anyhow::Result<ClosestNeighboursIterator<'a>> {
        ClosestNeighboursIterator::new(self)
    }

    /// Get the number of nodes in this list.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

/// An iterator over unique relations sorted by distance.
///
/// This iterator is produced by [`NodesList::iter_closest_neighbours`], and implements
/// the algorithm described in the documentation of [`NodesList`].
///
/// It yields unique [`Relation`]s between nodes in order of increasing distance by
/// lazy evaluation of nearest-neighbours using a KD-Tree.
///
/// ## Lifetime
///
/// Since this struct holds references to the underlying [`NodesList`], its lifetime
/// is tied to that of the [`NodesList`].
pub struct ClosestNeighboursIterator<'a> {
    list: &'a NodesList,
    generators: Vec<Box<dyn Iterator<Item = (NodeDistanceType, &'a usize)> + 'a>>,
    seen: HashSet<(usize, usize)>,

    /// A min-heap of relations sorted by distance.
    ///
    /// There are typically N-1 relations in this heap at any time; one for each node except
    /// the node with the highest nearest-neighbour distance. This is because A-B and B-A
    /// are considered the same relation, and we only store the one with the lower node ID
    /// first.
    ///
    /// When we pop a relation from this heap, we then fan out from the `node_a` of that
    /// relation to find its next nearest neighbour that hasn't already been paired with it,
    /// and push that new relation onto the heap, therefore maintaining one relation per node
    /// in the heap at all times (except the one with the highest nearest-neighbour distance).
    sorted_distances: BinaryHeap<Reverse<Relation>>,
}

impl<'a> ClosestNeighboursIterator<'a> {
    pub fn new(list: &'a NodesList) -> anyhow::Result<Self> {
        let length = list.len();

        let generators =
            list.nodes
                .iter()
                .map(
                    |node| -> anyhow::Result<
                        Box<dyn Iterator<Item = (NodeDistanceType, &usize)> + 'a>,
                    > {
                        let iter = list
                            .tree
                            .iter_nearest(node, &kdtree::distance::squared_euclidean)
                            .map_err(|e| {
                                anyhow::anyhow!(
                                    "Failed to compute nearest neighbors for node {:?}: {}",
                                    node,
                                    e
                                )
                            })?;
                        Ok(Box::new(iter) as Box<dyn Iterator<Item = (NodeDistanceType, &usize)>>)
                    },
                )
                .collect::<anyhow::Result<Vec<Box<_>>>>()?;

        let mut instance = Self {
            list,
            generators,
            seen: HashSet::new(),
            sorted_distances: BinaryHeap::new(),
        };

        // Since we can't move `seen` into a closure, we do this with a for loop.
        (0..length)
            .into_iter()
            .try_for_each(|node_id| instance.advance_generator_of(node_id).and(Ok(())))?;

        eprintln!(
            "Built NodesList with {} nodes and {} unique relations",
            length,
            instance.sorted_distances.len()
        );

        Ok(instance)
    }

    /// Get the number of nodes in the underlying list.
    pub fn nodes_list_len(&self) -> usize {
        self.list.len()
    }

    /// Internal function to advance the generator for a given node ID, pushing
    /// the next valid relation onto the heap.
    ///
    /// It will check that
    /// - the closest node is not itself,
    /// - the closest node has a lower ID than itself (to avoid duplicates), and
    /// - the pair has not already been seen.
    ///
    /// It returns [`Ok`] wrapping ``true`` if a new relation was pushed onto the heap,
    /// or ``false`` if the generator is exhausted.
    fn advance_generator_of(&mut self, node_id: usize) -> anyhow::Result<bool> {
        for (closest_distance, closest_node_id) in self.generators[node_id].by_ref() {
            if *closest_node_id == node_id {
                // Skip self
                continue;
            } else if *closest_node_id > node_id {
                // We only need the bottom half of the matrix, so we can stop here.
                continue;
            } else if self.seen.contains(&(node_id, *closest_node_id))
                || self.seen.contains(&(*closest_node_id, node_id))
            {
                // Already seen this pair
                continue;
            }

            #[cfg(feature = "trace")]
            {
                println!(
                    "Node {:?} closest to {:?} with distance {}",
                    self.list.nodes[node_id],
                    self.list.nodes[*closest_node_id],
                    closest_distance.sqrt()
                );
            }
            self.sorted_distances.push(Reverse(Relation {
                node_a: node_id,
                node_b: *closest_node_id,
                distance: closest_distance,
            }));
            self.seen.insert((node_id, *closest_node_id));

            return Ok(true);
        }

        Ok(false)
    }

    /// Pop the next closest relation from the heap.
    pub fn pop_next_relation(&mut self) -> Option<Relation> {
        let relation: Relation = self
            .sorted_distances
            .pop()
            .map(|rev_relation| rev_relation.0)?;

        self.advance_generator_of(relation.node_a).ok()?;

        Some(relation)
    }
}

impl Iterator for ClosestNeighboursIterator<'_> {
    type Item = Relation;

    fn next(&mut self) -> Option<Relation> {
        self.pop_next_relation()
    }
}

impl<'a> TryFrom<&'a NodesList> for ClosestNeighboursIterator<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a NodesList) -> Result<Self, Self::Error> {
        ClosestNeighboursIterator::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
                              57,618,57
                              906,360,560
                              592,479,940
                              352,342,300
                              466,668,158
                              542,29,236
                              431,825,988
                              739,650,466
                              52,470,668
                              216,146,977
                              819,987,18
                              117,168,530
                              805,96,715
                              346,949,466
                              970,615,88
                              941,993,340
                              862,61,35
                              984,92,344
                              425,690,689";

    #[test]
    fn test_build_nodes_heap_from_text() {
        let nodes_heap = NodesList::build_from_text(TEST_INPUT).unwrap();
        assert_eq!(nodes_heap.nodes.len(), 20);
    }

    #[test]
    fn test_get_node_by_id() {
        let nodes_heap = NodesList::build_from_text(TEST_INPUT).unwrap();
        assert_eq!(
            nodes_heap.get_node_by_id(0).unwrap(),
            &[162.0, 817.0, 812.0]
        );
        assert_eq!(
            nodes_heap.get_node_by_id(18).unwrap(),
            &[984.0, 92.0, 344.0]
        )
    }

    #[test]
    fn test_iterate_relations() {
        let expected = [
            ([425.0, 690.0, 689.0], [162.0, 817.0, 812.0]),
            ([431.0, 825.0, 988.0], [162.0, 817.0, 812.0]),
            ([805.0, 96.0, 715.0], [906.0, 360.0, 560.0]),
            ([425.0, 690.0, 689.0], [431.0, 825.0, 988.0]),
        ];

        let nodes_list = NodesList::build_from_text(TEST_INPUT).expect("Failed to build NodesList");
        let mut closest_neighbours_iter = nodes_list
            .iter_closest_neighbours()
            .expect("Failed to create ClosestNeighboursIterator");

        let iter = expected.iter();
        for (i, (expected_a, expected_b)) in iter.enumerate() {
            let relation = closest_neighbours_iter
                .next()
                .expect("Failed to get next relation");
            let node_a = nodes_list
                .get_node_by_id(relation.node_a)
                .expect("Failed to get find Node A");
            let node_b = nodes_list
                .get_node_by_id(relation.node_b)
                .expect("Failed to get find Node B");

            assert_eq!(
                (node_a, node_b),
                (expected_a, expected_b),
                "Failed at iteration {}",
                i
            );
            eprintln!("Passed iteration {}", i);
        }
    }
}
