use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;


pub fn read_input_lines(filename: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let digits = line
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        out.push(digits);
    }
    Ok(out)
}

fn find_index_of_max_value(values: &[u8]) -> usize {
    let mut idx: Vec<usize> = (0..values.len()).collect();
    idx.sort_by(
        |&i, &j| values[j].cmp(&values[i]),
    );
    idx[0]
}



pub fn read_input_joltage(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let digits = line
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        // Sort digits and obtain indexes "idx"
        // if the largest digit is not in the last index
        //  its a 1st digit, take its index and find the largest digit in
        //  digits[largest_idx..digits.len()]; this is the second digit
        // if the largest digit is the last index
        //  this is the second digit
        //  first digit is then the largest digit in digits[..digits.len()-1]
        // 1899 -> 9981
        // 0123 -> 2310
        let jolt: (u8, u8);
        let largest_digit_id = find_index_of_max_value(&digits);
        if largest_digit_id < digits.len()-1 {
            let second_largest_digit_id = find_index_of_max_value(
                &digits[largest_digit_id+1..],
            );
            jolt = (
                digits[largest_digit_id],
                digits[largest_digit_id+1..][second_largest_digit_id],
            );
        } else {
            let second_largest_digit_id = find_index_of_max_value(
                &digits[..digits.len()-1],
            );
            jolt = (
                digits[..digits.len()-1][second_largest_digit_id],
                digits[largest_digit_id],
            );
        }
        let joltage = jolt.0 * 10 + jolt.1;
        println!("Joltage: {joltage}");
        out.push(joltage);
    }
    Ok(out)
}