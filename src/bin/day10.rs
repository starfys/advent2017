#![feature(inclusive_range_syntax)]
extern crate advent2017;

use advent2017::KnotHasher;
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
    // Trim input
    let input: String = input.trim().to_string();
    // Parse input
    let lengths: Vec<u8> = input.split(',').map(|num| num.parse().unwrap()).collect();
    // Create a knot hasher
    let mut knot_hasher: KnotHasher = KnotHasher::new();
    knot_hasher.do_rounds(&lengths, 1);
    // Get some data out of the hasher
    let part1_answer: u16 = knot_hasher.bytes[0] as u16 * knot_hasher.bytes[1] as u16;
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", knot_hasher.knot_hash(&input));
}
