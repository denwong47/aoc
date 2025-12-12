pub mod models;

pub const SHAPE_COUNT: usize = 6;

mod input;
pub use input::INPUT;

mod progress;

fn main() {
    let (shape_builders, requirements) =
        models::parse_input::<SHAPE_COUNT>(INPUT).expect("Failed to parse input");

    #[cfg(not(feature = "compute"))]
    {
        println!(
            "\x1b[93mCheat mode enabled: only counting requirements that can possibly fit.\x1b[0m"
        );
        println!("\x1b[93mThis does NOT compute the actual solution!\x1b[0m");
        let can_fit = requirements
            .iter()
            .filter(|requirement| {
                requirement
                    .can_possibly_fit_using(&shape_builders)
                    .expect("Failed to check if requirement can fit")
            })
            .count();

        println!("Number of requirements that can possibly fit: {}", can_fit);
    }

    #[cfg(feature = "compute")]
    {
        use itertools::Itertools;
        use kdam::tqdm;

        println!("\x1b[92mComputing full solution...\x1b[0m");

        let shapes = shape_builders
            .into_iter()
            .flat_map(|builder| builder.build())
            .collect::<Vec<_>>();

        #[cfg(feature = "trace")]
        {
            eprintln!(
                "Total number of shapes generated: \x1b[36m{}\x1b[0m",
                shapes.len()
            );
        }

        for requirement in requirements {
            let placements = models::build_placements_for_requirement(&shapes, &requirement);

            println!(
                "For requirement on container {}x{} with shape counts {:?}, found \x1b[36m{}\x1b[0m possible placements.",
                requirement.container.width,
                requirement.container.height,
                requirement.shape_counts,
                placements.len()
            );
        }
    }
}
