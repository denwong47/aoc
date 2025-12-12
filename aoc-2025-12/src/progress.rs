use crate::models;

pub fn calculate_total_placements<const S: usize>(
    shapes: &[models::Shape],
    requirement: &models::Requirement<S>,
) -> usize {
    let count_per_shape = shapes.iter().fold([0; S], |mut acc, shape| {
        acc[shape.index] += 1;
        acc
    });

    let shape_by_counts_product = count_per_shape
        .iter()
        .zip(requirement.shape_counts.iter())
        .map(|(&available, &required)| {
            if required == 0 {
                0
            } else {
                available * required
            }
        })
        .sum::<usize>();

    shape_by_counts_product
        * (requirement.container.width - shapes[0].width() + 1)
        * (requirement.container.height - shapes[0].height() + 1)
}
