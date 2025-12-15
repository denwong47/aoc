//! ## Day 1: Secret Entrance
//! 
//! The Elves have good news and bad news.
//! 
//! The good news is that they've discovered project management! This has given them the
//! tools they need to prevent their usual Christmas emergency. For example, they now
//! know that the North Pole decorations need to be finished soon so that other critical
//! tasks can start on time.
//! 
//! The bad news is that they've realized they have a different emergency: according to
//! their resource planning, none of them have any time left to decorate the North Pole!
//! 
//! To save Christmas, the Elves need you to finish decorating the North Pole by
//! December 12th.
//! 
//! Collect stars by solving puzzles. Two puzzles will be made available on each day;
//! the second puzzle is unlocked when you complete the first. Each puzzle grants one
//! star. Good luck!
//! 
//! You arrive at the secret entrance to the North Pole base ready to start decorating.
//! Unfortunately, the password seems to have been changed, so you can't get in. A
//! document taped to the wall helpfully explains:
//! 
//! "Due to new security protocols, the password is locked in the safe below. Please see
//! the attached document for the new combination."
//! 
//! The safe has a dial with only an arrow on it; around the dial are the numbers 0
//! through 99 in order. As you turn the dial, it makes a small click noise as it
//! reaches each number.
//! 
//! The attached document (your puzzle input) contains a sequence of rotations, one per
//! line, which tell you how to open the safe. A rotation starts with an L or R which
//! indicates whether the rotation should be to the left (toward lower numbers) or to
//! the right (toward higher numbers). Then, the rotation has a distance value which
//! indicates how many clicks the dial should be rotated in that direction.
//! 
//! So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point
//! at 19. After that, a rotation of L19 would cause it to point at 0.
//! 
//! Because the dial is a circle, turning the dial left from 0 one click makes it point
//! at 99. Similarly, turning the dial right from 99 one click makes it point at 0.
//! 
//! So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95.
//! After that, a rotation of R5 could cause it to point at 0.
//! 
//! The dial starts by pointing at 50.
//! 
//! You could follow the instructions, but your recent required official North Pole
//! secret entrance security training seminar taught you that the safe is actually a
//! decoy. The actual password is the number of times the dial is left pointing at 0
//! after any rotation in the sequence.
//! 
//! For example, suppose the attached document contained the following rotations:
//! 
//!     L68 L30 R48 L5 R60 L55 L1 L99 R14 L82
//! 
//! Following these rotations would cause the dial to move as follows:
//! 
//!     The dial starts by pointing at 50.
//!     The dial is rotated L68 to point at 82.
//!     The dial is rotated L30 to point at 52.
//!     The dial is rotated R48 to point at 0.
//!     The dial is rotated L5 to point at 95.
//!     The dial is rotated R60 to point at 55.
//!     The dial is rotated L55 to point at 0.
//!     The dial is rotated L1 to point at 99.
//!     The dial is rotated L99 to point at 0.
//!     The dial is rotated R14 to point at 14.
//!     The dial is rotated L82 to point at 32.
//! 
//! Because the dial points at 0 a total of three times during this process, the password in this example is 3.
//! 
//! Analyze the rotations in your attached document. What's the actual password to open the door?
//! 
//! Your puzzle answer was 1145.
//! 
//! ## Part Two
//! 
//! You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.
//! 
//! As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:
//! 
//! "Due to newer security protocols, please use password method 0x434C49434B until further notice."
//! 
//! You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.
//! 
//! Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:
//! 
//!     The dial starts by pointing at 50.
//!     The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
//!     The dial is rotated L30 to point at 52.
//!     The dial is rotated R48 to point at 0.
//!     The dial is rotated L5 to point at 95.
//!     The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
//!     The dial is rotated L55 to point at 0.
//!     The dial is rotated L1 to point at 99.
//!     The dial is rotated L99 to point at 0.
//!     The dial is rotated R14 to point at 14.
//!     The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
//! 
//! In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.
//! 
//! Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!
//! 
//! Using password method 0x434C49434B, what is the password to open the door?
//! 
//! Your puzzle answer was 6561.
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **

mod input;
use input::INPUT;

fn instructions_from_string(s: &str) -> impl Iterator<Item = (char, u16)> + '_ {
    s.split_whitespace().map(|s| {
        let (dir, amt) = s.split_at(1);
        (
            dir.chars().next().expect("Invalid direction"),
            amt.parse::<u16>().expect("Invalid amount"),
        )
    })
}

/// A wheel (or dial) that can be rotated left or right, tracking how many times it
/// passes through and ends at position 0.
/// 
/// This implementation uses a generic constant parameter `S` to define the size of the
/// wheel, defaulting to ``100`` if not specified.
#[derive(Debug, PartialEq, Eq)]
pub struct Wheel<const S: u16 = 100> {
    pub position: u16,
    pub ends_at_zero_count: usize,
    pub pass_through_zero_count: usize,
}

impl<const S: u16> Wheel<S> {
    pub fn new(position: u16) -> Self {
        Self {
            position,
            ends_at_zero_count: 0,
            pass_through_zero_count: 0,
        }
    }

    pub fn set_position(&mut self, position: i32, direction: char) {
        let size = S as i32;

        let mut raw_position = position % size;
        let mut revolutions = (position / size).unsigned_abs() as u16;

        if raw_position <= 0 && (self.position > 0 && direction == 'L') {
            revolutions += 1;
        }
        if raw_position < 0 {
            raw_position += size;
        }

        assert!(raw_position >= 0);

        self.position = raw_position as u16;
        self.pass_through_zero_count += revolutions as usize;

        if self.position == 0 {
            self.ends_at_zero_count += 1;
        }
    }

    pub fn rotate(&mut self, direction: char, amount: u16) {
        let current_passes_through_zero = self.pass_through_zero_count;
        match direction {
            'L' => {
                self.set_position(self.position as i32 - amount as i32, direction);
            }
            'R' => {
                self.set_position(self.position as i32 + amount as i32, direction);
            }
            _ => {
                panic!("Invalid direction {:?}", direction);
            }
        }
        let suffix = if self.pass_through_zero_count > current_passes_through_zero {
            &format!(
                "; during this rotation, it points at 0 {} times(s).",
                self.pass_through_zero_count - current_passes_through_zero
            )
        } else {
            ""
        };
        eprintln!(
            "The dial is rotated {direction}{amount} to point at {position}{suffix}",
            position = self.position,
        );
    }

    pub fn execute(&mut self, instructions: impl Iterator<Item = (char, u16)>) {
        eprintln!("The dial starts by pointing at {}", self.position);
        for (direction, amount) in instructions {
            self.rotate(direction, amount);
        }
    }
}

fn main() {
    let mut wheel = Wheel::<100>::new(50);

    let instructions = instructions_from_string(INPUT);

    wheel.execute(instructions);

    println!(
        "The dial ends pointing at {} having passed through zero {} times and ended at zero {} times.",
        wheel.position, wheel.pass_through_zero_count, wheel.ends_at_zero_count
    );
}

#[cfg(test)]
mod tests_set_position {
    use super::*;

    macro_rules! create_test {
        ($name:ident(size=$size:literal, initial=$initial:literal, position=$position:literal, direction=$direction:literal, expected=$expected:expr)) => {
            #[test]
            fn $name() {
                let mut wheel: Wheel<$size> = Wheel::new($initial);

                wheel.set_position($position, $direction);

                assert_eq!(wheel, $expected);
            }
        };
    }

    create_test!(test1(
        size = 100,
        initial = 0,
        position = 249,
        direction = 'R',
        expected = Wheel::<100> {
            position: 49,
            ends_at_zero_count: 0,
            pass_through_zero_count: 2,
        }
    ));
    create_test!(test2(
        size = 100,
        initial = 0,
        position = -249,
        direction = 'L',
        expected = Wheel::<100> {
            position: 51,
            ends_at_zero_count: 0,
            pass_through_zero_count: 2,
        }
    ));
    create_test!(test3(
        size = 100,
        initial = 1,
        position = -249,
        direction = 'L',
        expected = Wheel::<100> {
            position: 51,
            ends_at_zero_count: 0,
            pass_through_zero_count: 3,
        }
    ));
    create_test!(test4(
        size = 100,
        initial = 0,
        position = 200,
        direction = 'R',
        expected = Wheel::<100> {
            position: 0,
            ends_at_zero_count: 1,
            pass_through_zero_count: 2,
        }
    ));
    create_test!(test5(
        size = 100,
        initial = 50,
        position = -100,
        direction = 'L',
        expected = Wheel::<100> {
            position: 0,
            ends_at_zero_count: 1,
            pass_through_zero_count: 2,
        }
    ));
}

#[cfg(test)]
mod tests_rotate {
    use super::*;

    macro_rules! create_test {
        ($name:ident(size=$size:literal, initial=$initial:literal, direction=$direction:literal, amount=$amount:literal, expected=$expected:expr)) => {
            #[test]
            fn $name() {
                let mut wheel: Wheel<$size> = Wheel::new($initial);

                wheel.rotate($direction, $amount);

                assert_eq!(wheel, $expected);
            }
        };
    }

    create_test!(test1(
        size = 100,
        initial = 0,
        direction = 'R',
        amount = 250,
        expected = Wheel::<100> {
            position: 50,
            ends_at_zero_count: 0,
            pass_through_zero_count: 2,
        }
    ));

    create_test!(test2(
        size = 100,
        initial = 0,
        direction = 'L',
        amount = 249,
        expected = Wheel::<100> {
            position: 51,
            ends_at_zero_count: 0,
            pass_through_zero_count: 2,
        }
    ));

    create_test!(test3(
        size = 100,
        initial = 1,
        direction = 'L',
        amount = 251,
        expected = Wheel::<100> {
            position: 50,
            ends_at_zero_count: 0,
            pass_through_zero_count: 3,
        }
    ));

    create_test!(test4(
        size = 100,
        initial = 50,
        direction = 'R',
        amount = 150,
        expected = Wheel::<100> {
            position: 0,
            ends_at_zero_count: 1,
            pass_through_zero_count: 2,
        }
    ));

    create_test!(test5(
        size = 100,
        initial = 50,
        direction = 'L',
        amount = 150,
        expected = Wheel::<100> {
            position: 0,
            ends_at_zero_count: 1,
            pass_through_zero_count: 2,
        }
    ));
}

#[cfg(test)]
mod tests_execute {
    use super::*;

    #[test]
    fn test1() {
        let mut wheel = Wheel::<100>::new(50);

        let instructions = instructions_from_string("L68 L30 R48 L5 R60 L55 L1 L99 R14 L82");

        wheel.execute(instructions);

        assert_eq!(
            wheel,
            Wheel::<100> {
                position: 32,
                ends_at_zero_count: 3,
                pass_through_zero_count: 6,
            }
        );
    }
}
