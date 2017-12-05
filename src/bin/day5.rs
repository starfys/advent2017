use std::io;
use std::io::prelude::*;
fn main() {
    // Get a handle on stdin;
    let stdin = io::stdin();
    // Read ints from stdin into a vector
    let original_offsets: Vec<isize> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect();
    // Part 1
    // Make a copy of the vector, since we will rewrite it
    let mut offsets = original_offsets.clone();
    // Set steps and offset
    let mut steps: usize = 0;
    let mut cur_offset: isize = 0;
    // Run through the program
    while (cur_offset as usize) < offsets.len() {
        // Store the current offset for when we jump
        let last_offset: isize = cur_offset;
        // Jump
        cur_offset += offsets[cur_offset as usize];
        // Increment the offset at the last offset
        offsets[last_offset as usize] += 1;
        // Count a step
        steps += 1;
    }
    println!("Part1: {}", steps);
    // Part 2
    // Make a copy of the vector, since we will rewrite it
    let mut offsets = original_offsets.clone();
    // Reset steps and offset
    steps = 0;
    cur_offset = 0;
    while (cur_offset as usize) < offsets.len() {
        // Store the current offset for when we jump
        let last_offset: isize = cur_offset;
        // Jump
        cur_offset += offsets[cur_offset as usize];
        // Increment the offset at the last offset
        if offsets[last_offset as usize] >= 3 {
            offsets[last_offset as usize] -= 1;
        } else {
            offsets[last_offset as usize] += 1;
        }
        // Count a step
        steps += 1;
    }
    println!("Part2: {}", steps);
}
