use crate::{SimpleGraphError, traits, wrapper};
use num_traits::Zero;
use std::{
    cmp::{Ord, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

/// Implements Dijkstra's algorithm to find the shortest path from a start node to a destination node.
pub fn dijkstra<'s, K, D, N>(
    start: &'s N,
    destination: &'s K,
    get_node_by_key: impl Fn(&K) -> Option<&'s N> + Clone,
) -> Result<(Vec<&'s K>, D), SimpleGraphError<K, D>>
where
    K: Debug + Clone + Eq + Hash + 's,
    D: Zero + Ord + Clone + Debug,
    N: traits::IsNode<'s, K, D>,
{
    let mut current_node = start;
    let mut visited_nodes: HashSet<&'s K> = HashSet::new();
    let mut unvisited_nodes: HashMap<&'s K, (&'s N, Vec<&'s K>, D)> = HashMap::new();
    let mut unvisited_distances: BinaryHeap<(Reverse<D>, wrapper::UnorderedItem<&'s K>)> =
        BinaryHeap::new();

    unvisited_nodes.insert(
        current_node.id(),
        (current_node, vec![current_node.id()], D::zero()),
    );

    loop {
        // Mark the current node as visited
        visited_nodes.insert(current_node.id());
        #[cfg(feature = "trace")]
        eprintln!("Visiting node {:?}", current_node.id());
        let (current_path, current_distance) = match unvisited_nodes.remove(current_node.id()) {
            Some((_, path, distance)) => Ok((path, distance)),
            None => Err(SimpleGraphError::Unreachable(format!(
                "Current node {:?} not in unvisited nodes",
                current_node.id()
            ))),
        }?;

        #[cfg(feature = "trace")]
        eprintln!(
            "Visiting node {:?} with current distance {:?} and path {:?}",
            current_node.id(),
            current_distance,
            current_path
        );

        // Stage 1 - Check if we reached the destination
        if current_node.id() == destination {
            return Ok((current_path, current_distance));
        }

        // Stage 2 - Update neighbours
        current_node
            .neighbours(get_node_by_key.clone())
            .try_for_each(|(neighbour_node, distance)| {
                let neighbour_id = neighbour_node.id();
                if distance < D::zero() {
                    return Err(SimpleGraphError::NegativeDistance {
                        start: current_node.id().clone(),
                        destination: neighbour_id.clone(),
                        distance: distance.clone(),
                    });
                }

                if visited_nodes.contains(neighbour_id) {
                    #[cfg(feature = "trace")]
                    eprintln!("Neighbour node {neighbour_id:?} already visited, skipping",);

                    return Ok(());
                }

                let new_distance = current_distance.clone() + distance.clone();
                unvisited_nodes
                    .entry(neighbour_id)
                    .and_modify(|(_, path, existing_distance)| {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Updating neighbour node {neighbour_id:?} with a shorter distance of {distance:?} (existing: {existing_distance:?})",
                        );

                        // Update the path and distance if the new distance is shorter
                        if new_distance < *existing_distance {
                            *existing_distance = new_distance.clone();

                            let mut new_path = current_path.clone();
                            new_path.push(neighbour_id);
                            std::mem::swap(path, &mut new_path);
                        }
                    })
                    .or_insert_with(|| {
                        #[cfg(feature = "trace")]
                        eprintln!(
                            "Adding new neighbour node {neighbour_id:?} with distance {distance:?}",
                        );
                        // Create a new entry for this neighbour if it doesn't exist
                        let mut new_path = current_path.clone();
                        new_path.push(neighbour_id);
                        (neighbour_node, new_path, new_distance.clone())
                    });

                // Push the new distance to the priority queue.
                // We do not check for existing entries here; they will be ignored when popped if outdated.
                unvisited_distances.push((
                    Reverse(new_distance),
                    wrapper::UnorderedItem::new(neighbour_id),
                ));

                Ok(())
            })?;

        // Stage 3 - Select the next current node
        loop {
            match unvisited_distances.pop() {
                Some((_, wrapper::UnorderedItem(neighbour_id))) => {
                    if visited_nodes.contains(neighbour_id) {
                        #[cfg(feature = "trace")]
                        eprintln!("Neighbour node {neighbour_id:?} already visited, skipping",);

                        continue;
                    }
                    if let Some((neighbour_node, _, _)) = unvisited_nodes.get(neighbour_id) {
                        current_node = *neighbour_node;
                        #[cfg(feature = "trace")]
                        eprintln!("Next current node set to {:?}", current_node.id());
                        break;
                    } else {
                        return Err(SimpleGraphError::Unreachable(format!(
                            "Neighbour node {:?} not found in unvisited nodes",
                            neighbour_id
                        )));
                    }
                }
                None => {
                    return Err(SimpleGraphError::Unreachable(format!(
                        "Destination node {:?} is unreachable from start node {:?}",
                        destination,
                        start.id()
                    )));
                }
            }
        }

        #[cfg(feature = "trace")]
        eprintln!(
            "Unvisited nodes remaining: {:?}",
            unvisited_nodes.keys().collect::<Vec<&&K>>()
        );
    }
}

#[cfg(test)]
mod tests_dijkstra {
    use super::*;
    use std::collections::HashMap;
    use crate::funcs::_tests::*;

    #[test]
    fn wiki_example() {
        let nodes: HashMap<u8, TestNode> = (1..=6).map(|id| (id, TestNode::new(id))).collect();

        let start_node = nodes.get(&1).expect("Start node not found");
        let destination_id = 5;
        let (path, distance) =
            dijkstra(start_node, &destination_id, |key| nodes.get(key)).expect("Dijkstra failed");

        assert_eq!(path, vec![&1, &3, &6, &5]);
        assert_eq!(distance, 20);
    }
}
