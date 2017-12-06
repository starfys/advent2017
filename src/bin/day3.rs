use std::io;

/// Solves day 3 part 1
fn solve_part1(input: u64) -> u64 {
    input
}
/// Solves day 3 part 2
fn solve_part2(input: u64) -> u64 {
    input
}
fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read the input into a buffer
    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();
    // Read the buffer as a number
    let input: u64 = input.trim().parse().unwrap();
    // Solve each part and print
    println!("{}", solve_part1(input));
    println!("{}", solve_part2(input));

}
