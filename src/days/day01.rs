use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::LinkedList;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let sol1: u64 = solve_part_one();
    let sol2: u64 = solve_part_two();
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part_one() -> u64 {
    let mut sol1: u64 = 0;
    let input = read_to_string("././input/input_day1.txt").expect("Error reading input file to string");
    for line in input.split("\n") {
        let mut numbers: LinkedList<u64> = LinkedList::new();
        let letters: Vec<char> = line.chars().collect();
        for char in letters {
            if char.is_numeric() {
                numbers.push_back(char.to_digit(10).expect("Error parsing character to integer!") as u64);
            }
        }
        if numbers.len() == 0 {
            continue;
        } else if numbers.len() == 1 {
            sol1 = sol1 + numbers.pop_front().expect("Error getting value from linked list")*11;
        } else {
            sol1 = sol1 + format!("{}{}", numbers.pop_front().expect("Error getting first value from linked list"), numbers.pop_back().expect("Error getting last value from linked list")).parse::<u64>().expect("Error parsing formatted string to u64");
        }
    }
    sol1
}
fn solve_part_two() -> u64 {
    let mut sol2: u64 = 0;
    let input = match read_to_string("././input/input_day1.txt") {
        Ok(input) => input,
        Err(error) => panic!("Problem reading the input file: {:?}", error),
    };
    for line in input.split("\n") {
        let mut numbers: LinkedList<u64> = LinkedList::new();
        let mut letters: Vec<char> = line.chars().collect();
        for char in &mut letters {
            char.make_ascii_lowercase();
        }
        for i in 0..letters.len() {
            if letters[i].is_numeric() {
                numbers.push_back(letters[i].to_digit(10).expect("Error parsing character to integer!") as u64);
            }
            if letters.len() >= i+3 && letters[i] == 'o' && letters[i+1] == 'n' && letters[i+2] == 'e' {
                numbers.push_back(1);
            }
            if letters[i] == 't' {
                if letters.len() >= i+3 && letters[i+1] == 'w' && letters[i+2] == 'o' {
                    numbers.push_back(2);
                }
                if letters.len() >= i+5 && letters[i+1] == 'h' && letters[i+2] == 'r' && letters[i+3] == 'e' && letters[i+4] == 'e' {
                    numbers.push_back(3);
                }
            }
            if letters.len() >= i+4 && letters[i] == 'f' {
                if letters[i+1] == 'o' && letters[i+2] == 'u' && letters[i+3] == 'r' {
                    numbers.push_back(4);
                }
                if letters[i+1] == 'i' && letters[i+2] == 'v' && letters[i+3] == 'e' {
                    numbers.push_back(5);
                }
            }
            if letters[i] == 's' {
                if letters.len() >= i+3 && letters[i+1] == 'i' && letters[i+2] == 'x' {
                    numbers.push_back(6);
                }
                if letters.len() >= i+5 && letters[i+1] == 'e' && letters[i+2] == 'v' && letters[i+3] == 'e' && letters[i+4] == 'n' {
                    numbers.push_back(7);
                }
            }
            if letters.len() >= i+5 && letters[i] == 'e' && letters[i+1] == 'i' && letters[i+2] == 'g' && letters[i+3] == 'h' && letters[i+4] == 't'{
                numbers.push_back(8);
            }

            if letters.len() >= i+4 && letters[i] == 'n' && letters[i+1] == 'i' &&  letters[i+2] == 'n' && letters[i+3] == 'e'{
                numbers.push_back(9);
            }
        }
        if numbers.len() == 0 {
            continue;
        } else if numbers.len() == 1 {
            sol2 = sol2 + numbers.pop_front().expect("Error getting value from linked list")*11;
        } else {
            sol2 = sol2 + format!("{}{}", numbers.pop_front().expect("Error getting first value from linked list"), numbers.pop_back().expect("Error getting last value from linked list")).parse::<u64>().expect("Error parsing formatted string to u64");
        }
    }
    sol2
}
