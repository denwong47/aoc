use super::CircuitOperation;
use fxhash::FxHashMap;

/// A tracker for which nodes are connected in the same circuit.
///
/// This supports merging circuits together and querying which circuit a node belongs to.
///
/// This keeps track of two maps:
///
/// - [`Self::node_to_circuit_map`] maps each node ID to its current circuit ID, and
/// - [`Self::circuit_to_nodes_map`] maps each circuit ID to the list of node IDs in that circuit.
///
/// They must be kept in sync when circuits are merged, hence this struct encapsulates both maps.
///
/// Each node starts in its own circuit, identified by its own node ID. i.e. the circuit ID
/// starts with ``(0, 0), (1, 1), (2, 2), ...``. When two nodes are [`Self::join`]ed,
/// all nodes in the circuit of ``node_b`` are moved to the circuit of `node_a`, which is
/// an ``O(1)`` lookup operation followed by an ``O(M)`` update operation, where ``M`` is the
/// number of nodes in the circuit of ``node_b``.
pub struct CircuitTracker {
    /// A map from node ID to circuit ID.
    node_to_circuit_map: FxHashMap<usize, usize>,
    circuit_to_nodes_map: FxHashMap<usize, Vec<usize>>,
}

impl CircuitTracker {
    /// To start with, each node is in its own circuit.
    pub fn with_capacity(capacity: usize) -> Self {
        CircuitTracker {
            node_to_circuit_map: FxHashMap::from_iter((0..capacity).map(|i| (i, i))),
            circuit_to_nodes_map: FxHashMap::from_iter((0..capacity).map(|i| (i, vec![i]))),
        }
    }

    /// Get the circuit ID for the given node.
    pub fn get_circuit_of(&self, node: usize) -> usize {
        self.node_to_circuit_map[&node]
    }

    /// Get a list of all nodes in the given circuit.
    pub fn get_nodes_in_circuit(&self, circuit_id: usize) -> Option<&Vec<usize>> {
        self.circuit_to_nodes_map.get(&circuit_id)
    }

    /// Merge the circuits containing the given nodes.
    ///
    /// All of the nodes in the circuit of `node_b` will be moved to the circuit of `node_a`.
    /// If the nodes are already in the same circuit, this is a no-op.
    pub fn join(&mut self, node_a: usize, node_b: usize) -> CircuitOperation {
        let circuit_a = self.node_to_circuit_map[&node_a];
        let circuit_b = self.node_to_circuit_map[&node_b];

        if circuit_a != circuit_b {
            let circuit_b_members = self
                .circuit_to_nodes_map
                .remove(&circuit_b)
                .expect("Circuit B should exist");
            let updated = circuit_b_members.len();

            circuit_b_members.iter().for_each(|&node_id| {
                self.node_to_circuit_map.insert(node_id, circuit_a);
            });
            self.circuit_to_nodes_map
                .get_mut(&circuit_a)
                .expect("Circuit A should exist")
                .extend(circuit_b_members);

            #[cfg(feature = "trace")]
            eprintln!(
                "Joined circuits {} (node {}) and {} (node {}) (updated {} nodes)",
                circuit_a, node_a, circuit_b, node_b, updated
            );

            CircuitOperation::Join {
                node_a,
                node_b,
                updated,
            }
        } else {
            #[cfg(feature = "trace")]
            eprintln!(
                "Nodes {node_a} and {node_b} are already joined in circuit {circuit_a} (0 nodes updated)",
            );
            CircuitOperation::NoOp { node_a, node_b }
        }
    }

    /// This is an O(N) operation that counts how many nodes are in the given circuit;
    /// For diagnostic purposes only, do not iterate in performance-sensitive code.
    /// See [`Self::circuits_by_size`] for a more efficient way to get sizes of all circuits.
    pub fn get_circuit_size(&self, circuit_id: usize) -> usize {
        self.circuit_to_nodes_map
            .get(&circuit_id)
            .map(|nodes| nodes.len())
            .unwrap_or(0)
    }

    /// Get the total number of unique circuits.
    pub fn total_circuits(&self) -> usize {
        self.circuit_to_nodes_map.len()
    }

    /// Get a list of circuits and their sizes, sorted by size descending.
    pub fn circuits_by_size(&self) -> Vec<(usize, usize)> {
        let mut counts_vec: Vec<(usize, usize)> = self
            .circuit_to_nodes_map
            .iter()
            .map(|(&circuit_id, nodes)| (circuit_id, nodes.len()))
            .collect();
        counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

        counts_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_circuit() {
        let tracker = CircuitTracker::with_capacity(10);

        for i in 0..10 {
            assert_eq!(tracker.get_circuit_of(i), i);
        }
    }

    #[test]
    fn test_join_once() {
        let mut tracker = CircuitTracker::with_capacity(10);

        let updated = tracker.join(1, 9);
        assert_eq!(
            updated,
            CircuitOperation::Join {
                node_a: 1,
                node_b: 9,
                updated: 1
            }
        );

        for i in 0..10 {
            let expected = if i == 9 { 1 } else { i };
            assert_eq!(
                tracker.get_circuit_of(i),
                expected,
                "node {} has the wrong circuit",
                i
            );
        }
    }

    #[test]
    fn test_join_multiple() {
        let mut tracker = CircuitTracker::with_capacity(10);
        assert_eq!(
            tracker.join(1, 9),
            CircuitOperation::Join {
                node_a: 1,
                node_b: 9,
                updated: 1
            }
        );
        assert_eq!(
            tracker.join(2, 9),
            CircuitOperation::Join {
                node_a: 2,
                node_b: 9,
                updated: 2
            }
        ); // Since we are joining 1 and 9 to 2, both 1 and 9 should now point to 2
        assert_eq!(
            tracker.join(3, 4),
            CircuitOperation::Join {
                node_a: 3,
                node_b: 4,
                updated: 1
            }
        );
        assert_eq!(
            tracker.join(5, 6),
            CircuitOperation::Join {
                node_a: 5,
                node_b: 6,
                updated: 1
            }
        );
        assert_eq!(
            tracker.join(4, 1),
            CircuitOperation::Join {
                node_a: 4,
                node_b: 1,
                updated: 3
            }
        ); // Joining circuits of 1 and 4 (which includes 2, 3, and 9, modifying only 1, 2 and 9)

        let expected_circuits = [0, 3, 3, 3, 3, 5, 5, 7, 8, 3];
        for i in 0..10 {
            assert_eq!(
                tracker.get_circuit_of(i),
                expected_circuits[i],
                "expected node {} to be in circuit {}",
                i,
                expected_circuits[i]
            );
        }
    }
}
