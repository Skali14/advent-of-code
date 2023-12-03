use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let input = read_to_string("././input/input_day2.txt").expect("Error reading input file to string");
    let sol1: u64 = solve_part_one(&input);
    let sol2: u64 = solve_part_two(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part_one(input: &String) -> u64 {
    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;
    let mut sol1: u64 = 0;
    let lines: Vec<&str> = input.lines().collect();

    'nextline : for i in 0..lines.len() {
        let splits: Vec<&str> = lines[i].split(';').collect();
        for j in 0..splits.len() {
            if splits[j].contains("blue") {
                let blue_index = splits[j].find("blue").expect("Blue was not found in this string");
                if splits[j].chars().nth(blue_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + splits[j].chars().nth(blue_index-2).unwrap().to_digit(10).unwrap() > total_blue {
                    continue 'nextline;
                }
            }

            if splits[j].contains("red") {
                let red_index = splits[j].find("red").expect("Red was not found in this string");
                if splits[j].chars().nth(red_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + splits[j].chars().nth(red_index-2).unwrap().to_digit(10).unwrap() > total_red {
                    continue 'nextline;
                }
            }

            if splits[j].contains("green") {
                let green_index = splits[j].find("green").expect("Green was not found in this string");
                    if splits[j].chars().nth(green_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + splits[j].chars().nth(green_index-2).unwrap().to_digit(10).unwrap() > total_green {
                        continue 'nextline;
                    }
            }
        }
        sol1 = sol1 + 1 + i as u64;
    }
    sol1
}

fn solve_part_two(input: &String) -> u64 {
    let mut sol2: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..lines.len() {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        let mut blue_line = lines[i];
        let mut blue_index = blue_line.find("blue").unwrap_or(0);
        while blue_index != 0 {
            let number = blue_line.chars().nth(blue_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + blue_line.chars().nth(blue_index-2).unwrap().to_digit(10).unwrap();
            if number > blue {
                blue = number;
            }
            blue_line = &blue_line[blue_index + 2..blue_line.len()];
            blue_index = blue_line.find("blue").unwrap_or(0);
        }

        let mut green_line = lines[i];
        let mut green_index = green_line.find("green").unwrap_or(0);
        while green_index != 0 {
            let number = green_line.chars().nth(green_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + green_line.chars().nth(green_index-2).unwrap().to_digit(10).unwrap();
            if number > green {
                green = number;
            }
            green_line = &green_line[green_index + 2..green_line.len()];
            green_index = green_line.find("green").unwrap_or(0);
        }

        let mut red_line = lines[i];
        let mut red_index = red_line.find("red").unwrap_or(0);
        while red_index != 0 {
            let number = red_line.chars().nth(red_index-3).unwrap().to_digit(10).unwrap_or(0)*10 + red_line.chars().nth(red_index-2).unwrap().to_digit(10).unwrap();
            if number > red {
                red = number;
            }
            red_line = &red_line[red_index + 2..red_line.len()];
            red_index = red_line.find("red").unwrap_or(0);
        }
        sol2 = sol2 + (red*blue*green);
    }
    sol2 as u64
}
