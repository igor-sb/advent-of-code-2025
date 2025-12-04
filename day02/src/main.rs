use std::io;

pub mod input_parser;

use input_parser::read_input_ranges;

fn main() -> Result<(), io::Error> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let input_file = format!("{cargo_root}/data/input.txt");
    println!("Input file: {input_file}");
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
