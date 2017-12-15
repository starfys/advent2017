use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Create a graph
    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();
    // Read in the graph
    for line in stdin.lock().lines() {
        // Unwrap the line
        let line = line.unwrap();
        // Split at the arrows to get the source/dest
        let line: Vec<&str> = line.split(" <-> ").collect();
        let src: u64 = line[0].parse().unwrap();
        let dsts: Vec<u64> = line[1]
            .split(", ")
            .map(|num| num.parse().unwrap())
            .collect();
        graph.insert(src, dsts);
    }
    // Makes 0 a special case
    let mut num_groups: usize = 0;
    while !graph.is_empty() {
        // Depth first search
        let mut visit_stack: Vec<u64> = Vec::with_capacity(graph.len());
        let mut visited: HashSet<u64> = HashSet::with_capacity(graph.len());
        if num_groups == 0 {
            visit_stack.push(0);
        } else {
            visit_stack.push(graph.keys().next().unwrap().clone());
        }
        while !visit_stack.is_empty() {
            let cur_node = visit_stack.pop().unwrap();
            if visited.contains(&cur_node) {
                continue;
            }
            visited.insert(cur_node);
            for node in graph.get(&cur_node).unwrap() {
                visit_stack.push(*node);
            }
        }
        // Remove all visited nodes from the graph
        for node in &visited {
            graph.remove(node);
        }
        if num_groups == 0 {
            println!("Part 1: {}", visited.len());
        }
        num_groups += 1;
    }
    println!("Part 2: {}", num_groups);
}
