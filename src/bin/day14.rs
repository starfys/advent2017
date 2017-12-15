#![feature(inclusive_range_syntax)]
extern crate advent2017;

use advent2017::KnotHasher;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;


// Converts [u8; 16] to its representation as bits [u8; 128]
fn bytes_to_bits(bytes: [u8; 16]) -> [u8; 128] {
    let mut bits: [u8; 128] = [0; 128];
    let mut index: usize = 0;
    for byte in bytes.iter() {
        let mut b: u8 = *byte;
        // Iterate over bits
        for _ in 0..8 {
            // Get rightmost bit
            bits[index] = if b & 0x80 == 0x80 { 1 } else { 0 };
            // Shift right
            b <<= 1;
            // Move to next index
            index += 1;
        }
    }
    bits
}
// Flood fills a grid
fn flood_fill(grid: &mut [[u8; 128]; 128], row: usize, col: usize, original: u8, replacement: u8) {
    // Initialize a stack
    let mut nodes_to_visit: VecDeque<(usize, usize)> = VecDeque::with_capacity(128);
    if grid[row][col] != original {
        return;
    }
    grid[row][col] = replacement;
    nodes_to_visit.push_back((row, col));
    while !nodes_to_visit.is_empty() {
        let (row, col) = nodes_to_visit.pop_front().unwrap();
        if row < grid.len() - 1 && grid[row + 1][col] == original {
            grid[row + 1][col] = replacement;
            nodes_to_visit.push_back((row + 1, col));
        }
        if row > 0 && grid[row - 1][col] == original {
            grid[row - 1][col] = replacement;
            nodes_to_visit.push_back((row - 1, col));
        }
        if col < grid[0].len() - 1 && grid[row][col + 1] == original {
            grid[row][col + 1] = replacement;
            nodes_to_visit.push_back((row, col + 1));
        }
        if col > 0 && grid[row][col - 1] == original {
            grid[row][col - 1] = replacement;
            nodes_to_visit.push_back((row, col - 1));
        }
    }
}

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
    // Get a knot hasher
    let mut knot_hasher: KnotHasher = KnotHasher::new();
    // Initialize a grid
    let mut disk_blocks: [[u8; 128]; 128] = [[0; 128]; 128];
    // Store number of blocks while we do this for part 1
    let mut num_blocks: usize = 0;
    // Calculate knot hashes for each row
    for row in 0..128 {
        disk_blocks[row] =
            bytes_to_bits(knot_hasher.dense_knot_hash(&format!("{}-{}", input, row)));
        if row < 128 {
            for bit in disk_blocks[row].iter() {
                num_blocks += *bit as usize;
            }
        }
    }
    println!("Part 1: {}", num_blocks);
    // Do a flood fill algorithm, and store a group per filled
    let mut num_groups: usize = 0;
    for row in 0..128 {
        for col in 0..128 {
            if disk_blocks[row][col] == 1 {
                flood_fill(&mut disk_blocks, row, col, 1, 0);
                num_groups += 1;
            }
        }
    }
    println!("Part 2: {}", num_groups);
}
