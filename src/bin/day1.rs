use std::io;

fn main() {
    // Read a line from stdin
    let mut captcha = String::new();
    match io::stdin().read_line(&mut captcha) {
        Ok(_) => {},
        Err(error) => panic!("Failed to read a line from stdin. Error: {}", error),
    }
    // Convert string to a vector of digits
    let captcha: Vec<u32> = captcha.trim().chars().map(|c| match c.to_digit(10) {
        Some(digit) => digit,
        None => 0
    }).collect();
    // Solve the first part
    // Won't be caught off-guard with a multiple-gigabyte file ;)
    let mut part1_sum: u64 = 0;
    let mut part2_sum: u64 = 0;
    // Iterate over the list and do both parts
    for (index, digit) in captcha.iter().enumerate() {
        // Check the next digit
        if *digit == captcha[(index + 1) % captcha.len()] {
            part1_sum += *digit as u64;
        }
        // Check the digit halfway across the list
        if *digit == captcha[(index + (captcha.len() / 2)) % captcha.len()] {
            part2_sum += *digit as u64;
        }
    }
    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
//TODO: build tests
