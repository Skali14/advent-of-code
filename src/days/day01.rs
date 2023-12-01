use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::LinkedList;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input = match read_to_string("././input/input_day1.txt") {
        Ok(input) => input,
        Err(error) => panic!("Problem reading the input file: {:?}", error),
    };

    for line in input.split("\n") {
        let mut numbers: LinkedList<u64> = LinkedList::new();
        for line_split in line.split("") {
            if line_split == "" {
                continue;
            }
            let char: char = line_split.parse().expect("Error getting char from line_split");
            if char.is_numeric() {
                numbers.push_back(line_split.parse().expect("Error parsing char to u64"));
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
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
