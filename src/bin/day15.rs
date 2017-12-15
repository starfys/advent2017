#![feature(test)]
extern crate test;

use std::io;
use std::io::prelude::*;
use test::Bencher;

// Solves part 1 for day 15
fn day15_part1(a: i64, b: i64) -> usize {
    // Initialize generators
    let mut gen_a: i64 = a;
    let mut gen_b: i64 = b;
    let mut result: usize = 0;
    // Run for 40 million iterations
    for _ in 0..40000000 {
        gen_a = (gen_a * 16807) % 2147483647;
        gen_b = (gen_b * 48271) % 2147483647;
        // Add 1 to result if there is a match
        result += (gen_a & 0xFFFF == gen_b & 0xFFFF) as usize;
    }
    result
}

// Solves part 2 for day 15
fn day15_part2(a: i64, b: i64) -> usize {
    // Initialize generators
    let mut gen_a: i64 = a;
    let mut gen_b: i64 = b;
    let mut result: usize = 0;
    // Run for 5 million iterations
    for _ in 0..5000000 {
        // Generate for A until a % 4 == 0
        // Skip the modulo, since it shouldn't be needed until the end
        loop {
            gen_a = (gen_a * 16807) % 2147483647;
            if gen_a % 4 == 0 {
                break;
            }
        }
        // Generate for B until B % 8 == 0
        // Skip the modulo, since it shouldn't be needed until the end
        loop {
            gen_b = (gen_b * 48271) % 2147483647;
            if gen_b % 8 == 0 {
                break;
            }
        }
        if gen_a & 0xFFFF == gen_b & 0xFFFF {
            result += 1;
        }
    }
    result
}

/// A generator that models the problem
struct Generator {
    value: i64,
    factor: i64,
}
impl Generator {
    /// Constructor
    fn new(start_value: i64, factor: i64) -> Generator {
        Generator {
            value: start_value,
            factor: factor,
        }
    }
}
impl Iterator for Generator {
    type Item = i64;
    /// Returns the next value from the generator
    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % 2147483647;
        Some(self.value)
    }
}

/// Solves part 1 with generators
fn day15_part1_generators(a: i64, b: i64) -> usize {
    let a: Generator = Generator::new(a, 16807);
    let b: Generator = Generator::new(b, 48271);
    // Calculate the number of matching values in 40M iterations
    a.zip(b)
        .take(40000000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}
/// Solves part 2 with generators
fn day15_part2_generators(a: i64, b: i64) -> usize {
    let a: Generator = Generator::new(a, 16807);
    let b: Generator = Generator::new(b, 48271);
    // Calculate the number of matching values in 40M iterations
    a.filter(|a| a % 4 == 0)
        .zip(b.filter(|b| b % 8 == 0))
        .take(5000000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read in the input
    let input: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap().split(" ").last().unwrap().parse().unwrap()
        })
        .collect();
    let a: i64 = input[0];
    let b: i64 = input[1];
    // Solve part 1
    println!("Part 1: {}", day15_part1(a, b));
    // Solve part 2
    println!("Part 2: {}", day15_part2(a, b));
}


#[cfg(test)]
mod day15 {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(day15_part1(65, 8921), 588);
    }
    #[test]
    fn test_part2() {
        assert_eq!(day15_part2(65, 8921), 309);
    }
    #[test]
    fn test_part1_generators() {
        assert_eq!(day15_part1_generators(65, 8921), 588);
    }
    #[test]
    fn test_part2_generators() {
        assert_eq!(day15_part2_generators(65, 8921), 309);
    }
    #[bench]
    fn bench_part1(bencher: &mut Bencher) {
        bencher.iter(|| day15_part1(65, 8921));
    }
    #[bench]
    fn bench_part2(bencher: &mut Bencher) {
        bencher.iter(|| day15_part2(65, 8921));
    }
    #[bench]
    fn bench_part1_generators(bencher: &mut Bencher) {
        bencher.iter(|| day15_part1_generators(65, 8921));
    }
    #[bench]
    fn bench_part2_generators(bencher: &mut Bencher) {
        bencher.iter(|| day15_part2_generators(65, 8921));
    }
}
