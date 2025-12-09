use crate::models::{Coord, IndexedCoords};

pub fn indexed_coords_from_text(input: &str) -> anyhow::Result<Vec<IndexedCoords>> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut parts = line
                .trim()
                .split(',')
                .map(|part| part.trim().parse::<Coord>());
            let x = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing x coordinate"))
                .and_then(|res| {
                    res.map_err(|e| anyhow::anyhow!("Failed to parse x coordinate: {}", e))
                })?;
            let y = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing y coordinate"))
                .and_then(|res| {
                    res.map_err(|e| anyhow::anyhow!("Failed to parse x coordinate: {}", e))
                })?;

            Ok(IndexedCoords::new(index, [x, y]))
        })
        .collect::<Result<Vec<_>, _>>()
}
