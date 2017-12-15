use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Used to store variables
    let mut variables: HashMap<String, i64> = HashMap::new();
    // Used to store part 2's answer
    let mut all_time_max: i64 = 0;
    // Get all lines as strings so our &str have something to point at
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    // Iterate over lines
    for line in lines {
        // Split the line
        let tokens: Vec<&str> = line.split(' ').collect();
        // Parse the token into variables
        // TODO: do this in nom
        let modify_var: &str = tokens[0];
        let modify_operation: &str = tokens[1];
        let modify_amount: i64 = tokens[2].parse().unwrap();
        let condition_var: &str = tokens[4];
        let condition_type: &str = tokens[5];
        let condition_value: i64 = tokens[6].parse().unwrap();
        // Check the condition
        let condition_result: bool = match condition_type {
            "<" => variables.get(condition_var).unwrap_or(&0) < &condition_value, 
            "<=" => variables.get(condition_var).unwrap_or(&0) <= &condition_value, 
            "==" => variables.get(condition_var).unwrap_or(&0) == &condition_value,
            "!=" => variables.get(condition_var).unwrap_or(&0) != &condition_value,
            ">" => variables.get(condition_var).unwrap_or(&0) > &condition_value,
            ">=" => variables.get(condition_var).unwrap_or(&0) >= &condition_value,
            _ => panic!("Invalid comparator"),
        };
        if condition_result {
            // Make sure the variable is initialized
            variables.entry(modify_var.to_string()).or_insert(0);
            // Do the operation
            match modify_operation {
                "inc" => *variables.get_mut(modify_var).unwrap() += modify_amount,
                "dec" => *variables.get_mut(modify_var).unwrap() -= modify_amount,
                _ => panic!("Invalid comparator"),
            }
            // When a value is modified, check it against the max
            all_time_max = all_time_max.max(*variables.get(modify_var).unwrap());
        }
    }
    // Calculate the max of all variables to get part 1's answer
    let final_max: i64 = *variables.values().max().unwrap();
    // Print the answers
    println!("Part 1: {}", final_max);
    println!("Part 2: {}", all_time_max);
}
