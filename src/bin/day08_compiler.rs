use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read in the input
    let program_lines: Vec<(String, String, i64, String, String, i64)> = stdin
        .lock()
        .lines()
        .map(|line| {
            // Unwrap the line
            let line: String = line.unwrap();
            // Split the line
            let line: Vec<String> = line.split(' ').map(|word| word.to_string()).collect();
            // Extract all useful vars
            (
                line[0].clone(),
                line[1].clone(),
                line[2].parse().unwrap(),
                line[4].clone(),
                line[5].clone(),
                line[6].parse().unwrap(),
            )
        })
        .collect();
    // Get a list of variables
    let mut vars: HashSet<String> = HashSet::new();
    for line in &program_lines {
        vars.insert(line.0.clone());
        vars.insert(line.3.clone());
    }
    // Change vars to a vector for later
    let vars: Vec<String> = vars.into_iter().collect();
    // Print the program's prelude
    println!("fn main() {{");
    // Print each variable as a declaration
    for var in &vars {
        println!("    let mut {}: i64 = 0;", var);
    }
    // Initialize a variable for part 2's answer
    println!("    let mut part2: i64 = 0;");
    // Print each program line as a program
    for line in &program_lines {
        println!("    if {} {} {} {{", line.3, line.4, line.5);
        println!(
            "        {} += {};",
            line.0,
            match line.1.as_str() {
                "inc" => line.2,
                "dec" => -line.2,
                _ => panic!("Invalid operation"),
            },
        );
        println!("    }}");
        // Check part 2
        println!("    if {} > part2 {{", line.0);
        println!("        part2 = {};", line.0);
        println!("    }}");
    }
    // Code to print what the largest value of all the variables is
    println!(
        "println!(\"Part 1: {{}}\", vec![{}].into_iter().max().unwrap());",
        &vars.join(",")
    );
    // Code to print the answer
    println!("println!(\"Part 2: {{}}\", part2);");
    // End the  program
    println!("}}");
}
