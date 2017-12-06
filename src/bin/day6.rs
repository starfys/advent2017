use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn redistribute(memory: &mut Vec<usize>) {
    // Determine the max value
    let (max_index, max_value): (usize, usize) = memory
        .iter()
        .cloned()
        .enumerate()
        .max_by(|&(_, x), &(_, y)| {
            // Custom comparator, since we want to use the first maximum
            if x < y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap();
    // Zero out the value at max index
    memory[max_index] = 0;
    // Redistribute the value among lower values
    // Each block will get max_value / len added
    // The first (max_value % len) get 1 added
    let distributed_value: usize = max_value / memory.len();
    let mut remaining_bonus_blocks: usize = max_value % memory.len();
    let mut remaining_blocks: usize = memory.len();
    let mut index = (max_index + 1) % memory.len();
    while remaining_blocks > 0 {
        if index != max_index && remaining_bonus_blocks > 0 {
            memory[index] += 1;
            remaining_bonus_blocks -= 1;
        }
        memory[index] += distributed_value;
        index = (index + 1) % memory.len();
        remaining_blocks -= 1;
    }
}


fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read in the data
    let mut memory: Vec<usize> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    // Create a set of seen states and the iteration where it was seen
    let mut states: HashMap<Vec<usize>, usize> = HashMap::new();
    // Use this for loop as a counter for iterations
    for cur_iteration in 0.. {
        // Check if the state has been seen before
        if states.contains_key(&memory) {
            // If the state has been seen, print the current step
            println!("Part 1: {}", cur_iteration);
            // Print the cycle length
            println!("Part 2: {}", cur_iteration - states.get(&memory).unwrap());
            // Exit the program
            break;
        } else {
            // Add the new memory state
            states.insert(memory.clone(), cur_iteration);
        }
        // Redistribute the array
        // ☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭☭
        redistribute(&mut memory);
    }
}
