use std::collections::LinkedList;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("././input/input_day4.txt").expect("Error reading input file to string");
    let sol1: u64 = solve_part_one(&input);
    let sol2: u64 = solve_part_two(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part_one(input: &String) -> u64 {
    let mut sol1 = 0;

    let lines: Vec<&str> = input.lines().collect();
    for i in 0..lines.len() {
        let mut winning_count = 0;
        let mut winning_numbers: LinkedList<u32> = LinkedList::new();
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 10..lines[i].find('|').unwrap() {
            if chars[j].is_numeric() {
                if chars[j+1].is_numeric() && !chars[j-1].is_numeric() {
                    winning_numbers.push_back(chars[j].to_digit(10).unwrap()*10 + chars[j+1].to_digit(10).unwrap());

                } else if !chars[j+1].is_numeric() && !chars[j-1].is_numeric() {
                    winning_numbers.push_back(chars[j].to_digit(10).unwrap());

                }
            }
        }
        for j in lines[i].find('|').unwrap()..chars.len() {
            if chars[j].is_numeric() {
                if j+1 < chars.len() && chars[j+1].is_numeric() && !chars[j-1].is_numeric(){
                    if winning_numbers.contains(&(chars[j].to_digit(10).unwrap() * 10 + chars[j + 1].to_digit(10).unwrap())) {
                        winning_count = winning_count + 1;
                    }
                } else if !chars[j-1].is_numeric() {
                    if winning_numbers.contains(&(chars[j].to_digit(10).unwrap())) {
                        winning_count = winning_count + 1;
                    }
                }
            }
        }
        sol1 = sol1 + 2_u64.pow(winning_count - 1);
    }
    sol1
}

fn solve_part_two(input: &String) -> u64 {

    0
}