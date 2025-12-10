//! Did a lot of thinking, have a working solution, but the solution requires 300+ button presses which
//! is beyond what I can reasonably compute with any algorithm I know.
//! 
//! All the LLMs just say "use MILP" but don't explain what it does or how it works.
//! 
//! We type in the problem and press a button and it gives an answer.
//! 
//! What fun. What joy. What achievement. What learning.
//! 
//! Not.

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod input;
pub mod models;
use input::INPUT;

fn main() {
    '_part1: {
        let part1 = INPUT
            .lines()
            .map(|line| {
                println!("Part 1 Processing line: {}", line);
                let machine = models::Machine::new_from_input(line).unwrap();
                let solution = machine.brute_force().expect("No solution found");
                solution.len()
            })
            .sum::<usize>();

        println!("Total buttons pressed across all machines: {}", part1);
    }

    #[cfg(feature = "milp")]
    '_part2: {
        let part2 = INPUT
            .lines()
            .map(|line| {
                println!("Part 2 Processing line: {}", line);
                let machine = models::Machine::new_from_input(line).unwrap();
                let solution = machine
                    .solve_milp(&machine.joltage.values)
                    .expect("No solution found");
                solution.len()
            })
            .sum::<usize>();

        println!(
            "Total buttons pressed across all machines (Part 2): {}",
            part2
        );
    }
}
