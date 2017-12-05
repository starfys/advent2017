use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

// Returns the solution for part 1 and part 2 of t
fn solve_day4(lines: Vec<String>) -> (usize, usize) {
    lines
        // Iterate over each line
        .into_iter()
        // Run a map function over this iterator
        .map(|line| {
            // Trim newlines off the line and get it as a list of string slices
            let line: Vec<&str> = line.trim().split(" ").collect();
            // Grab the line size. This will be used later
            let num_words: usize = line.len();
            // Build a hashset and test its size
            // If hashset.len == line.len, all words in the line are unique
            let num_permutations_unique: usize =
            HashSet::<Vec<char>>::from_iter(line.iter().map(|word| {
                // Get the word's characters
                let mut word_characters: Vec<char> = word.chars().collect();
                // Sort the vec
                word_characters.sort();
                // Return the vector
                word_characters
            })).len();
            // For this, we can just take ownership of the iterator, since it's no longer needed
            let num_unique: usize = HashSet::<&str>::from_iter(line.into_iter()).len();
            // Return whether all are unique for both cases
            (
                num_unique == num_words,
                num_permutations_unique == num_words,
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
    let input_lines: Vec<String> = stdin.lock().lines().flat_map(|line| line).collect();
    // Solve the problem
    let (part1_answer, part2_answer): (usize, usize) = solve_day4(input_lines);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
