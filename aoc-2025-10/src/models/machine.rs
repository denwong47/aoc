use itertools::Itertools;

use anyhow::Ok;

use crate::models::{CountArray, combination};
use fxhash::FxHashSet;

use super::{Button, Indicators, Joltage};

#[cfg(feature="milp")]
use good_lp::{variables, variable, default_solver, SolverModel, Solution, Variable, Expression};


#[cfg(feature = "progress")]
use kdam::{BarExt, tqdm};
#[cfg(feature = "progress")]
use std::io::{IsTerminal, stderr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Machine {
    pub indicators: Indicators,
    pub buttons: Vec<Button>,
    pub joltage: Joltage,
}

impl Machine {
    pub fn new(indicators: Indicators, buttons: Vec<Button>, joltage: Joltage) -> Self {
        Self {
            indicators,
            buttons,
            joltage,
        }
    }

    pub fn new_from_input(input: &str) -> anyhow::Result<Self> {
        let mut indicators: Option<Indicators> = None;
        let mut buttons: Vec<Button> = Vec::new();
        let mut joltage: Option<Joltage> = None;

        for segment in input.split_whitespace() {
            match segment.chars().next() {
                Some('[') => {
                    if indicators.is_none() {
                        indicators = Some(Indicators::new_from_input(segment)?);
                    } else {
                        anyhow::bail!("Multiple Indicators found in Machine input");
                    }
                }
                Some('(') => {
                    if let Some(indicators) = &indicators {
                        let length = indicators.len();
                        let button = Button::new_from_input(buttons.len(), segment, length)?;
                        buttons.push(button);
                    } else {
                        anyhow::bail!("Button found before Indicators in Machine input");
                    }
                }
                Some('{') => {
                    if joltage.is_none() {
                        joltage = Some(Joltage::new_from_input(segment)?);
                    } else {
                        anyhow::bail!("Multiple Joltage found in Machine input");
                    }
                }
                None => {}
                _ => {
                    anyhow::bail!("Could not parse Machine segment: {:?}", segment);
                }
            }
        }

        let instance = Self::new(
            indicators.ok_or_else(|| anyhow::anyhow!("No Indicators found in Machine input"))?,
            buttons,
            joltage.ok_or_else(|| anyhow::anyhow!("No Joltage found in Machine input"))?,
        );

        instance
            .buttons
            .iter()
            .enumerate()
            .map(|(button_id, button)| (format!("Button {button_id}"), button.len()))
            .chain(std::iter::once((
                "Indicators".to_owned(),
                instance.indicators.len(),
            )))
            .chain(std::iter::once((
                "Joltage".to_owned(),
                instance.joltage.len(),
            )))
            .try_fold(None, |expected_len: Option<(String, usize)>, (name, len)| {
                if let Some((source_name, expected)) = expected_len {
                    if expected != len {
                        anyhow::bail!(
                            "Mismatched lengths between {source_name} and {name}: expected {}, found {}",
                            expected,
                            len
                        );
                    }
                    Ok(Some((name, len)))
                } else {
                    Ok(Some((name, len)))
                }
            })?;

        Ok(instance)
    }

    /// Solve the Machine using Mixed Integer Linear Programming (MILP).
    /// 
    /// Or more specifically, call someone else's MILP solver to do the heavy lifting.
    /// I honestly have no idea how this works, we define the problem and call `solve()`
    /// and things just work. This is very depressing and defeating. Not cool.
    #[cfg(feature="milp")]
    pub fn solve_milp(&self, target: &CountArray<u16>) -> anyhow::Result<Vec<usize>> {
        let mut vars = variables!();
    
        // 1. Create a variable for each mask type (how many times to use it)
        // We assume masks are unique.
        let mask_counts: Vec<Variable> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer())) 
            .collect();

        // 2. Define the objective: Minimize total count
        let objective = mask_counts.iter().sum::<Expression>();
    
        let mut problem = vars.maximise(objective * -1) // Minimizing is maximizing negative
            .using(default_solver); // or specific solver

        // 3. Add constraints for each column (0 to N-1)
        for col_idx in 0..target.len() {
            let mut constraint_expr = Expression::from(0);
            for button in self.buttons.iter() {
                if button.effect.values[col_idx] {
                    constraint_expr += mask_counts[button.index];
                }
            }
            // The sum of masks at this column must equal target[col]
            problem.add_constraint(constraint_expr.eq(target.values[col_idx]));
        }

        // 4. Solve
        let solution = problem.solve().map_err(|e| anyhow::anyhow!("MILP solve error: {}", e))?;

        // 5. Extract solution
        let mut result: Vec<usize> = Vec::new();
        for (button_idx, var) in mask_counts.iter().enumerate() {
            let count = solution.value(*var) as usize;
            for _ in 0..count {
                result.push(button_idx);
            }
        }

        Ok(result)
    }

    /// Solve the Machine using a depth-first search approach.
    /// 
    /// This is optimized to avoid revisiting already explored combinations.
    pub fn solve_dfs(&self, target: &CountArray<u16>) -> anyhow::Result<Vec<usize>> {
        #[cfg(feature = "progress")]
        kdam::term::init(stderr().is_terminal());

        let first_combination =
            combination::ButtonCombination::new(&self.buttons, target, Vec::new())?;

        #[cfg(feature = "progress")]
        let mut pbar = {
            let total = first_combination.difference.distance() as usize;
            tqdm!(total = total, desc = "Solving Machine")
        };

        let mut visited: FxHashSet<Vec<usize>> = FxHashSet::default();

        let mut memo = vec![first_combination];
        while !memo.is_empty() {
            let last_memo = memo
                .last_mut()
                .expect("Unreachable, memo should have at least one element");

            #[cfg(feature = "progress")]
            {
                let progress = pbar.total - last_memo.difference.distance() as usize;
                pbar.set_description(format!(
                    "Current distance: {:>7}, length: {:>3}",
                    last_memo.difference.distance(),
                    last_memo.combination.len()
                ));
                pbar.update_to(progress)
                    .map_err(|e| anyhow::anyhow!("Failed to update progress bar: {}", e))?;
            }

            #[cfg(feature = "trace")]
            eprintln!(
                "Current combination: {:<40} difference: {:?} distance: {:?}",
                format!("{:?}", last_memo.combination),
                last_memo.difference,
                last_memo.difference.distance()
            );

            if let Some(next_combination_result) = last_memo.next_combination() {
                let next_combination = next_combination_result?;
                if let Some(solution) = next_combination.solution() {
                    let mut solution = solution.clone();
                    solution.sort();

                    #[cfg(feature = "progress")]
                    {
                        pbar.update_to(pbar.total)
                            .map_err(|e| anyhow::anyhow!("Failed to update progress bar: {}", e))?;
                    }
                    return Ok(solution);
                }

                let sorted_combination = {
                    let mut combo = next_combination.combination.clone();
                    combo.sort_unstable();
                    combo
                };
                if next_combination.is_dead_end() {
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "Dead-end reached for combination: {:<40}",
                        format!("{:?}", next_combination.combination),
                    );
                    visited.insert(sorted_combination);
                } else if !visited.contains(&sorted_combination) {
                    memo.push(next_combination);
                } else {
                    #[cfg(feature = "trace")]
                    eprintln!(
                        "Already visited combination: {:<40}",
                        format!("{:?}", next_combination.combination),
                    );
                }
            } else {
                // We exhausted all options from this combination,
                // so pop it off the memo stack.
                memo.pop();
            }
        }

        anyhow::bail!("No solution found for Machine for {:?}", target.values)
    }

    /// A quick and dirty brute-force solution to find the minimal button presses
    /// required to achieve the target indicators.
    ///
    /// This is so that we can move onto Part 2 to see if we can optimize the solutions
    /// together.
    pub fn brute_force(&self) -> anyhow::Result<Vec<usize>> {
        for count in 1..=self.buttons.len() {
            if let Some(solution) = self.brute_force_by_length(count)? {
                return Ok(solution);
            }
        }

        anyhow::bail!("No solution found for Machine")
    }

    fn brute_force_by_length(&self, count: usize) -> anyhow::Result<Option<Vec<usize>>> {
        for combo in self.buttons.iter().combinations(count) {
            let combined_effect = combo
                .iter()
                .map(|button| &button.effect)
                .try_fold(CountArray::new(self.indicators.len()), |acc, effect| {
                    acc.add(effect)
                })?;

            if combined_effect.mask() == self.indicators.values {
                return Ok(Some(combo.iter().map(|button| button.index).collect()));
            }
        }

        Ok(None)
    }

    pub fn combination_to_button_display<'s>(
        &self,
        combination: impl Iterator<Item = &'s usize>,
    ) -> String {
        let mut display = String::new();
        for button_id in combination {
            if !display.is_empty() {
                display.push_str(", ");
            }
            display.push_str(&self.buttons[*button_id].effect.display_as_tuple());
        }
        display
    }
}

#[cfg(test)]
mod test_parsing {
    use super::*;

    #[test]
    fn example_parsing() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = Machine::new_from_input(input).expect("Failed to parse Machine");

        assert_eq!(machine.indicators.values, ".###.#");
        let expected_buttons = [(0, "#####."), (1, "#..##."), (2, "###.##"), (3, ".##...")];
        for (i, (expected_index, expected_effect)) in expected_buttons.iter().enumerate() {
            let button = &machine.buttons[i];
            assert_eq!(button.index, *expected_index);
            assert_eq!(button.effect, *expected_effect);
        }
        assert_eq!(machine.joltage.values, vec![10, 11, 11, 5, 10, 5].into());
    }
}

#[cfg(test)]
mod test_solve {
    use super::*;

    macro_rules! create_test {
        ($name:ident($input:expr) = $expected:expr) => {
            #[test]
            #[cfg(feature="milp")]
            fn $name() {
                let machine = Machine::new_from_input($input).expect("Failed to parse Machine");
                let solution = machine
                    .solve_milp(&machine.joltage.values)
                    .expect("Failed to solve Machine");

                eprintln!(
                    "Solution found:    {}",
                    machine.combination_to_button_display(solution.iter())
                );
                let final_state = Button::combine(
                    solution.iter().map(|id| &machine.buttons[*id]),
                    machine.indicators.len(),
                )
                .expect("Failed to combine buttons");
                assert_eq!(
                    final_state, machine.joltage.values,
                    "Final state from solution does not match Machine joltage"
                );

                let expected: Vec<usize> = $expected;
                eprintln!(
                    "Expected sequence: {}",
                    machine.combination_to_button_display(expected.iter())
                );
                let expected_state = Button::combine(
                    expected.iter().map(|id| &machine.buttons[*id]),
                    machine.indicators.len(),
                ).expect("Failed to combine expected buttons");
                assert_eq!(expected_state, machine.joltage.values, "Final state from model answer does not match expected state");

                assert_eq!(solution, expected, "Solution does not match expected");
            }
        };
    }

    create_test!(
        test_example_1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}") =
            vec![0, 1, 1, 1, 1, 1, 3, 4, 4, 4]
            // vec![0, 1, 1, 1, 1, 3, 3, 4, 4, 5] // The listed answer is actually [0, 1, 1, 1, 3, 3, 3, 4, 5, 5] - but both yield the same final state, and our solver happens to find this one.
    );
    create_test!(
        test_example_2("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}") =
            vec![0, 0, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3]
    );
    create_test!(
        test_example_3("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}") =
            vec![0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3]
    );
    create_test!(
        test_input_1("[..#.] (1,2,3) (1,3) (0,3) {6,14,4,20}") =
            vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2]
    );
    create_test!(
        test_input_2("[...##.] (0,1,2,4,5) (0,2,5) (0,1,5) (0,2,3,4) (0,4) {29,14,21,4,18,21}") = 
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]
    );
}
