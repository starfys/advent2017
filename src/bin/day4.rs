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
            // Trim the line and create a split iterator
            let line: Vec<&str> = line.trim().split(" ").collect();
            // Build a hashset and
            HashSet::<&str>::from_iter(line.iter().cloned()).len() == line.len()
        })
        .fold(
            0,
            |sum, is_identical| sum + if is_identical { 1 } else { 0 },
        );
    println!("{:?}", result)
}
