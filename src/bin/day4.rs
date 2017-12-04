use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() {
    let stdin = io::stdin();
    // Map over stdin's lines
    let (part1_answer, part2_answer): (usize, usize) = stdin
        .lock()
        .lines()
        .map(|line| {
            // Unwrap the line
            let line: String = match line {
                Ok(line) => line,
                Err(error) => panic!("Failed to get line from stdin. Error: {}", error),
            };
            // Trim newlines off the line and get it as a list of string slices
            let line: Vec<&str> = line.trim().split(" ").collect();
            // Build a hashset and test its size
            // If hashset.len == line.len, all words in the line are unique
            let num_unique: usize = HashSet::<&str>::from_iter(line.iter().cloned()).len();
            let num_permutations_unique: usize = HashSet::<Vec<char>>::from_iter(line.iter().map(|word| {
                // Get the word's characters
                let mut word_characters: Vec<char> = word.chars().collect();
                // Sort the vec
                word_characters.sort();
                // Join the word characters
                // Return the vector
                word_characters
            })).len();
            // Return whether all are unique for both cases
            (num_unique == line.len(), num_permutations_unique == line.len())
        })
        // Reduce over the list to get the number
        // of lines where all words are unique
        .fold(
            (0, 0),
            |(part1_sum, part2_sum), (all_unique, all_permutations_unique)| 
            (
                part1_sum + if all_unique { 1 } else { 0 },
                part2_sum + if all_permutations_unique {1} else {0}
            )
        );
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
