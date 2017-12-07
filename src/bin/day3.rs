/*  Copyright (C) 2017 Steven Sheffey 

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
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
