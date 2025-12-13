use std::error::Error;

pub mod input_parser;


pub fn part1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let joltage_per_bank = input_parser::read_input_joltage(input_file)?;
    let total_joltage: u64 = joltage_per_bank.iter().copied().map(u64::from).sum();
    println!("{total_joltage}");
    Ok(())
}