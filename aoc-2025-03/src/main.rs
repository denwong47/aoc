//! ## Day 3: Lobby
//! 
//! You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.
//! 
//! "Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."
//! 
//! You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."
//! 
//! "But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."
//! 
//! There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:
//! 
//! ```text
//! 987654321111111
//! 811111111111119
//! 234234234234278
//! 818181911112111
//! ```
//! 
//! The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
//! 
//! You'll need to find the largest possible joltage each bank can produce. In the above example:
//! 
//! ```text
//!     In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
//!     In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
//!     In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
//!     In 818181911112111, the largest joltage you can produce is 92.
//! ```
//! 
//! The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
//! 
//! There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?
//! 
//! Your puzzle answer was 17694.
//! 
//! ## Part Two
//! 
//! The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.
//! 
//! Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.
//! 
//! The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.
//! 
//! Consider again the example from before:
//! 
//! ```text
//! 987654321111111
//! 811111111111119
//! 234234234234278
//! 818181911112111
//! ```
//! 
//! Now, the joltages are much larger:
//! 
//! ```text
//!     In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
//!     In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
//!     In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
//!     In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
//! ```
//! 
//! The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.
//! 
//! What is the new total output joltage?
//! 
//! Your puzzle answer was 175659236361660.
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **


use std::collections::VecDeque;
mod input;
use input::INPUT;

fn build_vec_deque_from_string(s: &str) -> VecDeque<u8> {
    VecDeque::from(s.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
}
trait HighestSequentialCombination {
    fn filter_to_highest_sequential_combination<const N: usize>(&mut self);
    fn build_int(&self) -> u64;
}

impl HighestSequentialCombination for VecDeque<u8> {
    fn filter_to_highest_sequential_combination<const N: usize>(&mut self) {
        let mut pos: usize = 0;
        loop {
            if pos == self.len() - 1 || self.len() <= N {
                break;
            }
            if self[pos] < self[pos + 1] {
                self.remove(pos);
                pos = pos.saturating_sub(1);
            } else {
                pos += 1;
            }
        }

        if self.len() > N {
            drop(self.split_off(N));
        }
    }

    fn build_int(&self) -> u64 {
        self.iter()
            .fold(0u64, |acc, &digit| acc * 10u64 + digit as u64)
    }
}

fn main() {
    let sum: u64 = INPUT
        .split_whitespace()
        .map(|line| {
            let mut vec_deque = build_vec_deque_from_string(line);
            vec_deque.filter_to_highest_sequential_combination::<12>();
            vec_deque.build_int()
        })
        .sum();

    println!("Sum of all highest sequential combinations: {}", sum);
}

#[cfg(test)]
mod test_highest_sequential_combination {
    use super::*;

    macro_rules! create_test {
        ($name:ident::<$size:literal>($values:expr) = $expected:expr) => {
            #[test]
            fn $name() {
                let mut vec_deque: VecDeque<u8> = build_vec_deque_from_string($values);
                vec_deque.filter_to_highest_sequential_combination::<$size>();
                let result: Vec<u8> = vec_deque.into();
                assert_eq!(result, $expected);
            }
        };
    }

    create_test! {
        test1::<3>("987654321111111") = vec![9, 8, 7]
    }
    create_test! {
        test2::<2>("123456789") = vec![8, 9]
    }
    create_test! {
        test3::<4>("543216789") = vec![6, 7, 8, 9]
    }
    create_test! {test4::<5>("1111122222333334444455555") = vec![5, 5, 5, 5, 5]}
    create_test! {test5::<1>("987654321") = vec![9]}
    create_test! {
        test6::<2>("811111111111119") = vec![8, 9]
    }
}

#[cfg(test)]
mod test_build_int {
    use super::*;

    #[test]
    fn test_build_int() {
        let vec_deque: VecDeque<u8> = build_vec_deque_from_string("123456789");
        let result = vec_deque.build_int();
        assert_eq!(result, 123456789u64);
    }
}
