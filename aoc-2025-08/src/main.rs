//! # Day 8: **Playground**
//!
//! Equipped with a new understanding of teleporter maintenance, you confidently step
//! onto the repaired teleporter pad.
//!
//! You rematerialize on an unfamiliar teleporter pad and find yourself in a vast
//! underground space which contains a giant playground!
//!
//! Across the playground, a group of Elves are working on setting up an ambitious
//! Christmas decoration project. Through careful rigging, they have suspended a large
//! number of small electrical junction boxes.
//!
//! Their plan is to connect the junction boxes with long strings of lights. Most of the
//! junction boxes don't provide electricity; however, when two junction boxes are
//! connected by a string of lights, electricity can pass between those two junction
//! boxes.
//!
//! The Elves are trying to figure out which junction boxes to connect so that
//! electricity can reach every junction box. They even have a list of all of the
//! junction boxes' positions in 3D space (your puzzle input).
//!
//! For example:
//!
//! ```text
//! 162,817,812
//! 57,618,57
//! 906,360,560
//! 592,479,940
//! 352,342,300
//! 466,668,158
//! 542,29,236
//! 431,825,988
//! 739,650,466
//! 52,470,668
//! 216,146,977
//! 819,987,18
//! 117,168,530
//! 805,96,715
//! 346,949,466
//! 970,615,88
//! 941,993,340
//! 862,61,35
//! 984,92,344
//! 425,690,689
//!```
//!
//! This list describes the position of 20 junction boxes, one per line. Each position
//! is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162,
//! Y=817, Z=812.
//!
//! To save on string lights, the Elves would like to focus on connecting pairs of
//! junction boxes that are as close together as possible according to
//! [straight-line distance]. In this example, the two junction boxes which are closest
//! together are 162,817,812 and 425,690,689.
//!
//! By connecting these two junction boxes together, because electricity can flow
//! between them, they become part of the same circuit. After connecting them, there is
//! a single circuit which contains two junction boxes, and the remaining 18 junction
//! boxes remain in their own individual circuits.
//!
//! Now, the two junction boxes which are closest together but aren't already directly
//! connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812
//! is already connected to another junction box, there is now a single circuit which
//! contains three junction boxes and an additional 17 circuits which contain one
//! junction box each.
//!
//! The next two junction boxes to connect are 906,360,560 and 805,96,715. After
//! connecting them, there is a circuit containing 3 junction boxes, a circuit
//! containing 2 junction boxes, and 15 circuits which contain one junction box each.
//!
//! The next two junction boxes are 431,825,988 and 425,690,689. Because these two
//! junction boxes were already in the same circuit, nothing happens!
//!
//! This process continues for a while, and the Elves are concerned that they don't have
//! enough extension cables for all these circuits. They would like to know how big the
//! circuits will be.
//!
//! After making the ten shortest connections, there are 11 circuits: one circuit which
//! contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits
//! which contain 2 junction boxes each, and seven circuits which each contain a single
//! junction box. Multiplying together the sizes of the three largest circuits (5, 4,
//! and one of the circuits of size 2) produces 40.
//!
//! Your list contains many junction boxes; connect together the 1000 pairs of junction
//! boxes which are closest together. Afterward, what do you get if you multiply
//! together the sizes of the three largest circuits?
//!
//! Your puzzle answer was 97384.
//!
//! ## Part Two
//!
//! The Elves were right; they definitely don't have enough extension cables. You'll
//! need to keep connecting junction boxes together until they're all in one large
//! circuit.
//!
//! Continuing the above example, the first connection which causes all of the junction
//! boxes to form a single circuit is between the junction boxes at 216,146,977 and
//! 117,168,530. The Elves need to know how far those junction boxes are from the wall
//! so they can pick the right extension cable; multiplying the X coordinates of those
//! two junction boxes (216 and 117) produces 25272.
//!
//! Continue connecting the closest unconnected pairs of junction boxes together until
//! they're all in the same circuit. What do you get if you multiply together the X
//! coordinates of the last two junction boxes you need to connect?
//!
//! Your puzzle answer was 9003685096.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **
//!
//! [straight-line distance]: https://en.wikipedia.org/wiki/Euclidean_distance

mod input;
pub mod models;
use input::INPUT;

#[cfg(feature = "profile")]
use std::time::Instant;

fn generate_circuit_map<'a>(
    iter_closest_neighbours: models::ClosestNeighboursIterator<'a>,
    steps: Option<usize>,
) -> anyhow::Result<(models::CircuitTracker, Vec<models::CircuitOperation>)> {
    let length = iter_closest_neighbours.nodes_list_len();
    let mut circuit_tracker = models::CircuitTracker::with_capacity(length);

    let mut op_history = Vec::with_capacity(steps.unwrap_or(length));
    let mut step_count = 0;
    for models::Relation { node_a, node_b, .. } in iter_closest_neighbours {
        let op = circuit_tracker.join(node_a, node_b);

        op_history.push(op);

        step_count += 1;

        if let Some(max_steps) = steps
            && step_count >= max_steps
        {
            break;
        }

        if circuit_tracker.total_circuits() <= 1 {
            break;
        }
    }

    Ok((circuit_tracker, op_history))
}

fn main() {
    let nodes_list =
        models::NodesList::build_from_text(INPUT).expect("failed to build nodes list from input");

    #[cfg(feature = "profile")]
    let start_time = Instant::now();
    '_part1: {
        let iter_closest_neighbours = nodes_list
            .iter_closest_neighbours()
            .expect("failed to build nodes heap");

        let (circuit_tracker, _) = generate_circuit_map(iter_closest_neighbours, Some(1000))
            .expect("failed to generate circuit map");

        println!(
            "Part 1: {}",
            circuit_tracker
                .circuits_by_size()
                .into_iter()
                .take(3)
                .fold(1, |acc, (_, size)| acc * size)
        );
    }
    #[cfg(feature = "profile")]
    {
        let duration = Instant::now() - start_time;
        eprintln!("Execution time: {:?}", duration);
    }

    #[cfg(feature = "profile")]
    let start_time = Instant::now();
    '_part2: {
        let iter_closest_neighbours = nodes_list
            .iter_closest_neighbours()
            .expect("failed to build nodes heap");
        let (_, full_op_history) = generate_circuit_map(iter_closest_neighbours, None)
            .expect("failed to generate full circuit map");

        let (node_a, node_b) = full_op_history
            .iter()
            .rev()
            .find_map(|op| match op {
                models::CircuitOperation::Join { node_a, node_b, .. } => {
                    let node_a_coords = nodes_list
                        .get_node_by_id(*node_a)
                        .expect("node_a not found");
                    let node_b_coords = nodes_list
                        .get_node_by_id(*node_b)
                        .expect("node_b not found");

                    Some((*node_a_coords, *node_b_coords))
                }
                _ => None,
            })
            .expect("no join operations found");

        println!("Part 2 nodes: {:?} and {:?}", node_a, node_b);
        println!("Part 2 answer: {:?}", node_a[0] as u64 * node_b[0] as u64);
    }
    #[cfg(feature = "profile")]
    {
        let duration = Instant::now() - start_time;
        eprintln!("Execution time: {:?}", duration);
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
    fn test_example() {
        let nodes_list = models::NodesList::build_from_text(TEST_INPUT).unwrap();
        let iter_closest_neighbours = nodes_list
            .iter_closest_neighbours()
            .expect("failed to build nodes heap");
        let (circuit_tracker, _) = generate_circuit_map(iter_closest_neighbours, Some(10))
            .expect("failed to generate circuit map");

        let expected_counts = vec![5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1];

        for (circuit_id, size) in circuit_tracker.circuits_by_size() {
            eprintln!("circuit {} has size {}", circuit_id, size);
        }
        assert_eq!(
            circuit_tracker
                .circuits_by_size()
                .into_iter()
                .map(|(_, size)| size)
                .collect::<Vec<usize>>(),
            expected_counts
        )
    }
}
