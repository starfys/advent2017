use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn solve_day4(lines: &Vec<String>) -> (usize, usize) {
    // Map over stdin's lines
    lines
        .iter()
        .map(|line| {
            // Trim newlines off the line and get it as a list of string slices
            let line: Vec<&str> = line.trim().split(" ").collect();
            // Build a hashset and test its size
            // If hashset.len == line.len, all words in the line are unique
            let num_unique: usize = HashSet::<&str>::from_iter(line.iter().cloned()).len();
            let num_permutations_unique: usize =
            HashSet::<Vec<char>>::from_iter(line.iter().map(|word| {
                // Get the word's characters
                let mut word_characters: Vec<char> = word.chars().collect();
                // Sort the vec
                word_characters.sort();
                // Return the vector
                // Only need to count unique vectors, so don't bother converting back to a string
                word_characters
            })).len();
            // Return whether all are unique for both cases
            (
                num_unique == line.len(),
                num_permutations_unique == line.len(),
            )
        })
        .fold((0, 0), |(num_unique_rows, num_permuted_unique_rows),
         (is_unique, is_permuted_unique)| {
            (
                num_unique_rows + is_unique as usize,
                num_permuted_unique_rows + is_permuted_unique as usize,
            )
        })
}

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Get all lines from stdin
    let input_lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(error) => panic!("Failed to read line. Error: {}.", error),
        })
        .collect();
    // Solve the problem
    let (part1_answer, part2_answer): (usize, usize) = solve_day4(&input_lines);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
