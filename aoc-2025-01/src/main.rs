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
