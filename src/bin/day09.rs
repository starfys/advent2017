use std::io;
use std::io::prelude::*;

fn main() {
    // Get a handle on stdin
    let mut stdin = io::stdin();
    // Read the entire input
    let mut input: String = String::new();
    match stdin.read_to_string(&mut input) {
        Ok(_) => {} 
        Err(error) => panic!("Failed to read input from stdin. Error: {}.", error),
    };
    // Create an iterator over characters
    let mut input = input.chars();
    // Store depth
    let mut depth: usize = 0;
    // Store score
    let mut score: usize = 0;
    // Store garbage
    let mut garbage_score: usize = 0;
    // Iterate over characters
    loop {
        let cur_char = match input.next() {
            Some(character) => character,
            None => break,
        };
        match cur_char {
            '{' => depth += 1,
            '}' => {
                score += depth;
                depth -= 1;
            }
            '<' => {
                loop {
                    let trash_char = input.next().unwrap();
                    if trash_char == '>' {
                        break;
                    }
                    if trash_char == '!' {
                        input.next().unwrap();
                        continue;
                    }
                    garbage_score += 1;
                }
            }
            '!' => {
                // Skip a character
                input.next().unwrap();
            }
            _ => {}
        };
    }
    println!();
    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage_score);
}
