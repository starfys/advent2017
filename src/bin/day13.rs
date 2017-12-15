use std::io;
use std::io::prelude::*;

struct Layer {
    depth: i64,
    range: i64,
}
impl Layer {
    fn severity(&self) -> i64 {
        self.depth * self.range
    }
    fn scanner_position(&self, time: i64) -> i64 {
        (self.range - 1) - (time % (2 * (self.range - 1)) - (self.range - 1)).abs()
    }
}
fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read in the ranges
    let firewall: Vec<Layer> = stdin
        .lock()
        .lines()
        .map(|line| {
            // Unwrap the line
            let line = line.unwrap();
            // Split at the arrows to get the source/dest
            let line: Vec<i64> = line.split(": ").map(|num| num.parse().unwrap()).collect();
            // Get depth and range
            Layer {
                depth: line[0],
                range: line[1],
            }
        })
        .collect();
    for start_time in 0.. {
        // Run a packet through the scanner
        let mut time: i64 = start_time;
        let mut total_severity: i64 = 0;
        let mut packet_depth: i64 = 0;
        for layer in &firewall {
            // Iterate however many nanoseconds it takes between the previous layer and the current one
            time += layer.depth - packet_depth;
            packet_depth = layer.depth;
            // If the scanner is at 0
            if layer.scanner_position(time) == 0 {
                total_severity += layer.severity();
            }
        }
        if start_time == 0 {
            println!("Part 1: {}", total_severity);
        }
        if total_severity == 0 && firewall[0].scanner_position(start_time) != 0 {
            println!("Part 2: {}", start_time);
            break;
        }
    }
}
