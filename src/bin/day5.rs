#![feature(test)]
extern crate test;
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

// Benchmarks
#[cfg(test)]
mod day5 {
    use super::*;
    use test::Bencher;
    #[test]
    /// Tests part1
    fn test_part1() {
        assert_eq!(solve_part1(&vec![0, 3, 0, 1, -3]), 5);
    }
    #[test]
    /// Tests part2
    fn test_part2() {
        assert_eq!(solve_part2(&vec![0, 3, 0, 1, -3]), 10);
    }
    #[bench]
    /// Benchmarks part1
    fn bench_part1(bencher: &mut Bencher) {
        bencher.iter(|| solve_part1(&vec![0, 3, 0, 1, -3]))
    }

    #[bench]
    /// Benchmarks part2
    fn bench_part2(bencher: &mut Bencher) {
        bencher.iter(|| solve_part2(&vec![0, 3, 0, 1, -3]))
    }
}
