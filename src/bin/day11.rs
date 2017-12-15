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
    // Parse the input
    let input: Vec<&str> = input.trim().split(",").collect();
    // Initialize coordinates
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut cur_distance: i64 = 0;
    let mut max_distance: i64 = 0;
    for direction in input {
        let (dx, dy): (i64, i64) = match direction {
            "nw" => (-1, 1),
            "n" => (0, 1),
            "ne" => (1, 0),
            "se" => (1, -1),
            "s" => (0, -1),
            "sw" => (-1, 0),
            &_ => panic!("Invalid instruction"),
        };
        x += dx;
        y += dy;
        // Calculate distance from origin
        cur_distance = if x.signum() == y.signum() {
            (x + y).abs()
        } else {
            x.abs().max(y.abs())
        };
        // Set max distance appropriately
        max_distance = max_distance.max(cur_distance);
    }
    println!("Part 1: {}", cur_distance);
    println!("Part 2: {}", max_distance);
}
