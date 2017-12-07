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
#![feature(test)]
extern crate rand;
extern crate test;
use std::io;

/// Converts a string of numbers to a vector of digits
fn string_to_digits(input: &str) -> Vec<u32> {
    input
        // Iterate over string's characters
        .chars()
        // Convert each char to a number
        .map(|c| match c.to_digit(10) {
            Some(digit) => digit,
            // Just use 0 for errors
            None => 0
        })
        .collect()
}
/// Solves part 1 of the puzzle
fn solve_part1(captcha: &[u32]) -> u64 {
    // Store sum as u64 so we have more space
    let mut sum: u64 = 0;
    // Iterate over the list and do both parts
    for (index, digit) in captcha.iter().enumerate() {
        // Check the next digit
        if *digit == captcha[(index + 1) % captcha.len()] {
            sum += u64::from(*digit);

        }
    }
    // Return the sum
    sum
}

/// Solves part 2 of the puzzle
fn solve_part2(captcha: &[u32]) -> u64 {
    // Store sum as u64 so we have more space
    let mut sum: u64 = 0;
    // Iterate over the list and do both parts
    for (index, digit) in captcha.iter().enumerate() {
        // Check the next digit
        if *digit == captcha[(index + (captcha.len() / 2)) % captcha.len()] {
            sum += u64::from(*digit);
        }
    }
    // Return the sum
    sum
}
fn main() {
    // Read a line from stdin
    let mut captcha = String::new();
    match io::stdin().read_line(&mut captcha) {
        Ok(_) => {}
        Err(error) => panic!("Failed to read a line from stdin. Error: {}", error),
    }
    // Convert string to a vector of digits
    let captcha: Vec<u32> = string_to_digits(captcha.trim());
    // Solve the first part
    let part1_answer: u64 = solve_part1(&captcha);
    // Solve the second part
    let part2_answer: u64 = solve_part2(&captcha);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}


#[cfg(test)]
mod day1 {
    use rand;
    use rand::Rng;
    use super::{string_to_digits, solve_part1, solve_part2};
    use test::Bencher;

    /// The static test size for benchmarks
    const TEST_INPUT_SIZE: usize = 10;

    /// Used to generate random inputs for benchmarks
    fn generate_random_input<R: Rng>(rng: &mut R, size: usize) -> Vec<u32> {
        (0..size).map(|_| rng.gen_range(0, 10)).collect()
    }

    #[test]
    /// Test the digit converter
    fn test_string_to_digits() {
        // Check normal case
        assert_eq!(
            string_to_digits("123456789"),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        // Check empty case
        assert_eq!(string_to_digits(""), vec![]);
        // Check invalid characters
        assert_eq!(string_to_digits("aaa"), vec![0, 0, 0]);
        // Check invalid in mixture
        assert_eq!(
            string_to_digits("123abc456"),
            vec![1, 2, 3, 0, 0, 0, 4, 5, 6]
        );
    }
    #[test]
    /// Test solution for part 1
    fn test_part1() {
        // Examples from the problem
        assert_eq!(solve_part1(&string_to_digits("1122")), 3);
        assert_eq!(solve_part1(&string_to_digits("1111")), 4);
        assert_eq!(solve_part1(&string_to_digits("1234")), 0);
        assert_eq!(solve_part1(&string_to_digits("91212129")), 9);
    }

    #[test]
    /// Test solution for part 2
    fn test_part2() {
        // Examples from the problem
        assert_eq!(solve_part2(&string_to_digits("1212")), 6);
        assert_eq!(solve_part2(&string_to_digits("1221")), 0);
        assert_eq!(solve_part2(&string_to_digits("123425")), 4);
        assert_eq!(solve_part2(&string_to_digits("123123")), 12);
        assert_eq!(solve_part2(&string_to_digits("12131415")), 4);
    }

    #[bench]
    /// Bechmarks solution for part1
    fn bench_part1(bencher: &mut Bencher) {
        // Create an rng
        let mut rng = rand::thread_rng();
        // Benchmark tests
        bencher.iter(|| {
            solve_part1(&generate_random_input(&mut rng, TEST_INPUT_SIZE))
        });
    }

    #[bench]
    /// Bechmarks solution for part2
    fn bench_part2(bencher: &mut Bencher) {
        // Create an rng
        let mut rng = rand::thread_rng();
        // Benchmark tests
        bencher.iter(|| {
            solve_part2(&generate_random_input(&mut rng, TEST_INPUT_SIZE))
        });
    }
}
