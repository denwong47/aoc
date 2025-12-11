use crate::traits;

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
}

impl TestNode {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl<'s> traits::IsNode<'s, u8, u32> for TestNode {
    fn id(&self) -> &u8 {
        &self.id
    }

    fn neighbours(
        &'s self,
        get_node_by_key: impl Fn(&u8) -> Option<&'s Self>,
    ) -> impl Iterator<Item = (&'s Self, u32)> {
        CONNECTIONS
            .iter()
            .filter_map(move |(start, end, distance)| {
                if *start == self.id {
                    get_node_by_key(end).map(|node| (node, *distance))
                } else {
                    None
                }
            })
    }
}
