use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() {
    let stdin = io::stdin();
    // Map over stdin's lines
    let result: usize = stdin
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
            HashSet::<&str>::from_iter(line.iter().cloned()).len() == line.len()
        })
        // Reduce over the list to get the number
        // of lines where all words are unique
        .fold(
            0,
            |sum, is_unique| sum + if is_unique { 1 } else { 0 },
        );
    println!("{:?}", result)
}
