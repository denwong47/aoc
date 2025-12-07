use crate::types::*;

const SOURCE_INTENSITY: BeamIntensity = 1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputType {
    Passthrough,
    Source,
    Splitter,
}

impl TryFrom<char> for InputType {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(InputType::Passthrough),
            'S' => Ok(InputType::Source),
            '^' => Ok(InputType::Splitter),
            _ => anyhow::bail!("Invalid input type character: {}", c),
        }
    }
}

impl InputType {
    pub fn operate_on(
        &self,
        position: usize,
        source_intensity: &BeamIntensity,
        map: &mut BeamIntensityMap,
    ) {
        match self {
            InputType::Passthrough => {
                if source_intensity > &0 {
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "Beam at position {} with intensity {} passes through.",
                        position, source_intensity
                    );

                    let entry = map.entry(position).or_insert(0);
                    *entry += source_intensity;
                } else {
                    #[cfg(feature = "trace")]
                    eprintln!("No beam at position {}, nothing to pass through.", position);
                }
            }
            InputType::Source => {
                let entry = map.entry(position).or_insert(0);
                #[cfg(feature = "trace")]
                {
                    eprintln!(
                        "Source at position {} adds intensity {}. Previous intensity: {}",
                        position, SOURCE_INTENSITY, *entry
                    );
                }
                *entry += SOURCE_INTENSITY;
            }
            InputType::Splitter => {
                // This defines how the intensity upon split is handled.
                // Depending on what's on Part 2, we might need to divide the source or modify this.
                let result_intensity = source_intensity;
                if position > 0 {
                    let entry_left = map.entry(position - 1).or_insert(0);
                    *entry_left += result_intensity;
                }

                let entry_right = map.entry(position + 1).or_insert(0);
                *entry_right += result_intensity;
            }
        }
    }
}

pub fn calculate_beam_intensity_map(
    input: &str,
    intensity_map: Option<&BeamIntensityMap>,
) -> anyhow::Result<(SplitterHitCount, BeamIntensityMap)> {
    let mut iterator: Box<dyn Iterator<Item = (usize, BeamIntensity)>> =
        if let Some(source_map) = intensity_map {
            Box::new(source_map.iter().map(|(pos, inten)| (*pos, *inten)))
        } else {
            Box::new((0..input.len()).map(|i| (i, 0)))
        };

    iterator.try_fold(
        (
            SplitterHitCount::default(),
            default_intensity_map(input.len()),
        ),
        |(mut splitter_hit_count, mut result_map), (position, source_intensity)| {
            let input_type_char = input
                .chars()
                .nth(position)
                .ok_or_else(|| anyhow::anyhow!("Position out of bounds"))?;
            let input_type = InputType::try_from(input_type_char).map_err(|e| {
                anyhow::anyhow!("Error parsing input type at position {}: {}", position, e)
            })?;

            #[cfg(feature = "trace")]
            {
                eprintln!(
                    "Processing position {}: input type {:?}, source intensity {}",
                    position, input_type, source_intensity
                );
            }

            // Count splitter hits only if there is a beam present
            if input_type == InputType::Splitter && source_intensity > 0 {
                splitter_hit_count += 1;
            }

            // Operate on the input type
            input_type.operate_on(position, &source_intensity, &mut result_map);

            Ok((splitter_hit_count, result_map))
        },
    )
}

pub fn scan_input(input: &str) -> anyhow::Result<(SplitterHitCount, Option<BeamIntensityMap>)> {
    // This `step_by(2)` assumes that every second line is empty as per the input;
    // This saves around 40% of time but assumes well-formed input.
    input.lines().step_by(2).try_fold(
        (0u32, None),
        |(splitter_hit_count_acc, intensity_map_acc), line| {
            calculate_beam_intensity_map(line, intensity_map_acc.as_ref()).map(
                |(splitter_hit_count, intensity_map)| {
                    (
                        splitter_hit_count_acc + splitter_hit_count,
                        Some(intensity_map),
                    )
                },
            )
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_calculate_beam_intensity_map() {
        let (splitter_hit_count, intensity_map) = scan_input(INPUT).expect("Failed to scan input");

        assert_eq!(splitter_hit_count, 21);

        assert_eq!(
            intensity_map
                .expect("Failed to get intensity map")
                .values()
                .sum::<BeamIntensity>(),
            40
        );
    }
}
