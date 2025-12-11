use crate::traits::*;

pub const CONNECTIONS: &[(u8, u8, u32)] = &[
    (1, 2, 7),
    (1, 3, 9),
    (1, 6, 14),
    (2, 3, 10),
    (2, 4, 15),
    (3, 4, 11),
    (3, 6, 2),
    (4, 5, 6),
    (6, 5, 9),
];

pub struct TestNode {
    id: u8,
    neighbours: Vec<(u8, u32)>,
}

impl TestNode {
    pub fn new(id: u8, neighbours: Vec<(u8, u32)>) -> Self {
        Self { id, neighbours }
    }

    pub fn new_with_connections(id: u8, connection: &[(u8, u8, u32)]) -> Self {
        let neighbours = connection
            .iter()
            .filter_map(|(start, end, distance)| {
                if *start == id {
                    Some((*end, *distance))
                } else {
                    None
                }
            })
            .collect::<Vec<(u8, u32)>>();

        Self::new(id, neighbours)
    }
}

impl<'s> IsNode<'s, u8, u32> for TestNode {
    fn id(&self) -> &u8 {
        &self.id
    }

    fn neighbours(
        &'s self,
        get_node_by_key: impl Fn(&u8) -> Option<&'s Self>,
    ) -> impl Iterator<Item = (&'s Self, u32)> {
        self.neighbours.iter().map(move |(neighbour_id, distance)| {
            let neighbour_node =
                get_node_by_key(neighbour_id).expect("Neighbour node not found in get_node_by_key");
            (neighbour_node, *distance)
        })
    }
}

impl<'s> IsNodeWithIndexedNeighbours<'s, u8, u32> for TestNode {
    fn get_neighbour(
        &'s self,
        index: usize,
        get_node_by_key: impl Fn(&u8) -> Option<&'s Self>,
    ) -> Option<(&'s Self, u32)> {
        // VERY POOR IMPLEMENTATION FOR TESTING PURPOSES ONLY - NEVER USE `self.neighbours().nth()` IN `get_neighbour`,
        // always do it the other way around for performance reasons.
        self.neighbours.get(index).map(|(neighbour_id, distance)| {
            let neighbour_node =
                get_node_by_key(neighbour_id).expect("Neighbour node not found in get_node_by_key");
            (neighbour_node, *distance)
        })
    }
}
