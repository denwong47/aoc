use super::{Requirement, ShapeBuilder};

pub fn parse_input<const S: usize>(
    input: &str,
) -> anyhow::Result<(Vec<ShapeBuilder>, Vec<Requirement<S>>)> {
    let mut lines = input.lines();
    let shapes = (0..S)
        .map(|index| {
            let shape = ShapeBuilder::from_lines(&mut lines)?;
            if shape.index != index {
                anyhow::bail!("Expected shape index {}, but found {}", index, shape.index);
            }
            Ok(shape)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let requirements = lines
        .skip_while(|line| line.trim().is_empty())
        .map(|line| Requirement::from_input(line))
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok((shapes, requirements))
}
