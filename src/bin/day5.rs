use std::io;
use std::io::prelude::*;

// Solves part 1 of day 5
fn solve_part1(offsets: &[isize]) -> usize {
    // Make a copy of the vector, since we will rewrite it
    let mut offsets: Vec<isize> = offsets.to_vec();
    // Set steps and offset
    let mut steps: usize = 0;
    let mut cur_offset: isize = 0;
    let mut last_offset: isize;
    // Run through the program
    while cur_offset >= 0 && (cur_offset as usize) < offsets.len() {
        // Store the current offset for when we jump
        last_offset = cur_offset;
        // Sanity check
        // Jump
        cur_offset += offsets[cur_offset as usize];
        // Increment the offset at the last offset
        offsets[last_offset as usize] += 1;
        // Count a step
        steps += 1;
    }
    // Returns steps
    steps
}

// Solves part 2 of day 5
fn solve_part2(offsets: &[isize]) -> usize {
    // Make a copy of the vector, since we will rewrite it
    let mut offsets: Vec<isize> = offsets.to_vec();
    // Set steps and offset
    let mut steps: usize = 0;
    let mut cur_offset: isize = 0;
    let mut last_offset: isize;
    // Run through the program
    while cur_offset >= 0 && (cur_offset as usize) < offsets.len() {
        // Store the current offset for when we jump
        last_offset = cur_offset;
        // Sanity check
        // Jump
        cur_offset += offsets[cur_offset as usize];
        // Increment or decrement the offset at the last offset
        if offsets[last_offset as usize] >= 3 {
            offsets[last_offset as usize] -= 1;
        } else {
            offsets[last_offset as usize] += 1;
        }
        // Count a step
        steps += 1;
    }
    // Returns steps
    steps
}
fn main() {
    // Get a handle on stdin;
    let stdin = io::stdin();
    // Read ints from stdin into a vector
    let original_offsets: Vec<isize> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect();
    println!("Part1: {}", solve_part1(&original_offsets));
    println!("Part2: {}", solve_part2(&original_offsets));
}
