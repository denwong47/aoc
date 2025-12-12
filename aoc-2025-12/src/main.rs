#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod models;

pub const SHAPE_COUNT: usize = 6;

mod input;
pub use input::INPUT;

mod progress;
mod solve;

#[cfg(test)]
mod _test;

fn main() {
    let (shape_builders, requirements) =
        models::parse_input::<SHAPE_COUNT>(INPUT).expect("Failed to parse input");

    #[cfg(feature = "cheat")]
    let requirements_that_can_be_fulfilled = {
        println!(
            "\x1b[93mCheat mode enabled: only counting requirements that can possibly fit.\x1b[0m"
        );
        println!("\x1b[93mThis does NOT compute the actual solution!\x1b[0m");
        let can_fit = requirements
            .iter()
            .enumerate()
            .filter_map(|(requirement_id, requirement)| {
                requirement
                    .can_possibly_fit_using(&shape_builders)
                    .expect("Failed to check if requirement can fit")
                    .then(|| requirement_id)
            })
            .collect::<Vec<_>>();

        println!(
            "Number of requirements that can possibly fit: {}",
            can_fit.len()
        );

        can_fit
    };

    #[cfg(feature = "compute")]
    {
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

        for (requirement_index, requirement) in requirements.into_iter().enumerate() {
            let placements = models::build_placements_for_requirement(&shapes, &requirement);

            println!(
                "For requirement on container {}x{} with shape counts {:?}, found \x1b[36m{}\x1b[0m possible placements.",
                requirement.container.width,
                requirement.container.height,
                requirement.shape_counts,
                placements.len()
            );
            let can_fulfill = solve::find_one_fulfillment(&requirement, &placements)
                .expect("Failed to determine if requirement can be fulfilled");

            println!(
                "\x1b[1mCalculated:\x1b[0m Requirement #{} fulfillment result: \x1b[{}m{}\x1b[0m",
                requirement_index,
                if can_fulfill.is_some() { "32" } else { "31" },
                format!("{:?}", can_fulfill)
            );
            #[cfg(feature = "cheat")]
            {
                let should_fulfill =
                    requirements_that_can_be_fulfilled.contains(&requirement_index);
                println!(
                    "\x1b[1mAnswer:    \x1b[0m Requirement #{} fulfillment result: \x1b[{}m{}\x1b[0m",
                    requirement_index,
                    if should_fulfill { "32" } else { "31" },
                    should_fulfill
                );
            }

            break;
        }
    }
}
