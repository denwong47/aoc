use std::str::Lines;

use itertools::Itertools;

pub type InnerShape = [bool; 9];

pub const FILLED_DISPLAY: char = '█';
pub const EMPTY_DISPLAY: char = '░';

fn rotate_right(inner_shape: &InnerShape) -> InnerShape {
    [
        inner_shape[6],
        inner_shape[3],
        inner_shape[0],
        inner_shape[7],
        inner_shape[4],
        inner_shape[1],
        inner_shape[8],
        inner_shape[5],
        inner_shape[2],
    ]
}

fn flip_horizontal(inner_shape: &InnerShape) -> InnerShape {
    [
        inner_shape[2],
        inner_shape[1],
        inner_shape[0],
        inner_shape[5],
        inner_shape[4],
        inner_shape[3],
        inner_shape[8],
        inner_shape[7],
        inner_shape[6],
    ]
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder {
    pub index: usize,
    inner_shape: InnerShape,
}

impl ShapeBuilder {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            inner_shape: [false; 9],
        }
    }

    pub fn count(&self) -> usize {
        self.inner_shape.iter().filter(|&&b| b).count()
    }

    pub fn from_lines(lines: &mut Lines) -> anyhow::Result<Self> {
        let mut iter_lines = lines.skip_while(|line| line.trim().is_empty());

        let index_line = iter_lines.next().ok_or_else(|| {
            anyhow::anyhow!("Expected index line at the start of input, but found EOF")
        })?;
        if !index_line.ends_with(':') {
            anyhow::bail!(
                "Expected index line to end with ':', but found: {}",
                index_line
            );
        }

        let index: usize = index_line[..index_line.len() - 1]
            .trim()
            .parse()
            .or_else(|_| anyhow::bail!("Failed to parse index from line: {}", index_line))?;

        let inner_shape = lines
            .take(3)
            .flat_map(|line| {
                let trimmed_line = line.trim();
                (0..=2).into_iter().map(|col_idx| -> anyhow::Result<bool> {
                    match trimmed_line.chars().nth(col_idx) {
                        Some('#') => Ok(true),
                        Some('.') => Ok(false),
                        Some(ch) => {
                            anyhow::bail!("Unexpected character in shape definition: {}", ch)
                        }
                        None => anyhow::bail!(
                            "Unexpected line length in shape definition: {}",
                            trimmed_line.len()
                        ),
                    }
                })
            })
            .enumerate()
            .try_fold(
                [false; 9],
                |mut acc, (idx, res)| -> anyhow::Result<InnerShape> {
                    let value = res?;
                    acc[idx] = value;
                    Ok(acc)
                },
            )?;

        Ok(Self { index, inner_shape })
    }

    pub fn build(self) -> Vec<Shape> {
        let shapes = [true, false]
            .into_iter()
            .cartesian_product(0..4)
            .fold(
                (fxhash::FxHashSet::default(), Vec::with_capacity(8)),
                |(mut seen, mut shapes), (flipped, rotations)| {
                    let mut current_shape = if flipped {
                        flip_horizontal(&self.inner_shape)
                    } else {
                        self.inner_shape
                    };

                    for _ in 0..rotations {
                        current_shape = rotate_right(&current_shape);
                    }

                    if seen.insert((current_shape, flipped)) {
                        shapes.push(Shape {
                            index: self.index,
                            rotations: rotations as u8,
                            flipped,
                            inner_shape: current_shape,
                        });
                    }

                    (seen, shapes)
                },
            )
            .1;

        #[cfg(feature = "trace")]
        {
            eprintln!(
                "From \x1b[36mShapeBuilder #{}\x1b[0m generated shapes:",
                self.index
            );
            let width = 9; // "ROT  270 | FLIP   0 "

            (0..5).for_each(|row| {
                eprint!("\u{2502} ");
                for shape in shapes.iter() {
                    match row {
                        0 => {
                            let colour = shape.rotations + 32;
                            eprint!(
                                "ROT  \x1b[{colour}m{:>3}\x1b[0m ",
                                (shape.rotations as u16) * 90
                            );
                        }
                        1 => {
                            let colour = if shape.flipped { 31 } else { 32 };
                            eprint!(
                                "FLIP \x1b[{colour}m{:>3}\x1b[0m ",
                                if shape.flipped { 1 } else { 0 }
                            );
                        }
                        2 | 3 | 4 => {
                            eprint!("{:^width$}", shape.display_line(row - 2));
                        }
                        _ => {
                            unreachable!()
                        }
                    };
                    eprint!("\u{2502} ");
                }
                eprintln!();
            });
            eprintln!();
        }
        shapes
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub index: usize,
    pub rotations: u8,
    pub flipped: bool,
    inner_shape: InnerShape,
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                let ch = if self.inner_shape[idx] {
                    FILLED_DISPLAY
                } else {
                    EMPTY_DISPLAY
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Shape {
    pub fn ansi_colour_code(&self) -> u8 {
        (18 + (self.index * 36 + self.rotations as usize) % 215) as u8
    }

    pub fn display_char(&self) -> char {
        if !self.flipped { '▒' } else { '▓' }
    }

    pub fn display_filled(&self) -> String {
        format!(
            "\x1b[38:5:{}m{}\x1b[0m",
            self.ansi_colour_code(),
            self.display_char()
        )
    }

    pub fn width(&self) -> usize {
        3
    }
    pub fn height(&self) -> usize {
        3
    }
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.inner_shape[y * 3 + x]
    }
    pub fn display_line(&self, row: usize) -> String {
        self.inner_shape[row * 3..(row + 1) * 3]
            .iter()
            .map(|&b| {
                if b {
                    self.display_filled()
                } else {
                    EMPTY_DISPLAY.to_string()
                }
            })
            .collect::<String>()
    }
}
