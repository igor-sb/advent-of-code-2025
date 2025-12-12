use std::io;

pub mod input_parser;

use input_parser::read_input_ranges;

pub fn part1(input_file: &str) -> Result<(), io::Error> {
    let ranges = read_input_ranges(&input_file)?;
    let mut invalid_numbers: Vec<u64> = Vec::new();
    for (start, end) in ranges {
        // Generate a full range
        for num in start..end+1 {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                let mid = num_str.len() / 2;
                if &num_str[..mid] == &num_str[mid..] {
                    invalid_numbers.push(num);
                }
            }
        }
    }
    let invalid_numbers_sum: u64 = invalid_numbers.iter().sum();
    println!("Sum of all invalid numbers: {invalid_numbers_sum}");
    Ok(())
}

pub fn part2(input_file: &str) -> Result<(), io::Error> {
    let ranges = read_input_ranges(&input_file)?;
    let mut invalid_numbers: Vec<u64> = Vec::new();
    for (start, end) in ranges {
        println!("Checking range: {start} - {end}");
        for num in start..end+1 {
            let num_str = num.to_string();
            // Check for repetitive numbers by concate
            let num_str_cat = format!("{}{}",
                &num_str[1..], &num_str[..num_str.len()-1]);
                // println!("   Generated: {num_str}");
            if num_str_cat.contains(&num_str) {
                invalid_numbers.push(num);
                // println!("   Repetitive number: {num_str}");
            }
        }
    }
    let invalid_numbers_sum: u64 = invalid_numbers.iter().sum();
    println!("Sum of all invalid numbers: {invalid_numbers_sum}");
    Ok(())
}