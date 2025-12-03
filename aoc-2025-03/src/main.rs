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
                if pos > 0 {
                    pos -= 1;
                }
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
