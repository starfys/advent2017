use std::io;
use std::io::prelude::*;
/// Solves part 1 of day 2
fn solve_part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}
/// Helper function for part 2
/// Returns a / b for which a,b are members of row and a % b == 0 and a != b
fn evenly_divisible_difference(row: &[i64]) -> i64 {
    for a in row {
        for b in row {
            if a % b == 0 && a != b {
                return a / b;
            }
        }
    }
    // If no values are found return 0
    0
}
/// Solves part 2 of day 2
fn solve_part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| evenly_divisible_difference(&row))
        .sum()
}

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read stdin into a list of lists
    let input_data: Vec<Vec<i64>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();
    // Solve part1
    println!("Part 1: {}", solve_part1(&input_data));
    println!("Part 2: {}", solve_part2(&input_data));
}
