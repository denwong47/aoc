use crate::models;

pub const SHAPE_COUNT: usize = 6;
pub const EXAMPLE_INPUT: &str = "0:
                                 ###
                                 ##.
                                 ##.
                             
                                 1:
                                 ###
                                 ##.
                                 .##
                             
                                 2:
                                 .##
                                 ###
                                 ##.
                             
                                 3:
                                 ##.
                                 ###
                                 ##.
                             
                                 4:
                                 ###
                                 #..
                                 ###
                             
                                 5:
                                 ###
                                 .#.
                                 ###
                             
                                 4x4: 0 0 0 0 2 0
                                 12x5: 1 0 1 0 2 2
                                 12x5: 1 0 1 0 3 2";

pub fn build_all_components<'r>(
    requirement_id: usize,
) -> (Vec<models::Shape>, models::Requirement<SHAPE_COUNT>) {
    let (shape_builders, mut requirements) =
        models::parse_input::<SHAPE_COUNT>(EXAMPLE_INPUT).expect("Failed to parse input");

    let shapes = shape_builders
        .into_iter()
        .flat_map(|builder| builder.build())
        .collect::<Vec<_>>();

    let requirement = requirements.remove(requirement_id);

    (shapes, requirement)
}
