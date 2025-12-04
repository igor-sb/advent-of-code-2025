use std::fs;
use std::io;

fn input_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

pub fn read_input_ranges(filename: &str) -> Result<Vec<(u64, u64)>, io::Error> {
    let file_contents = fs::read_to_string(filename)
        .map_err(|_| input_error(&format!("Cannot read filename: {filename}")))?;
    let items: Result<Vec<(u64, u64)>, io::Error> = file_contents
        .trim()
        .split(',')
        .map(String::from)
        .map(|pair| {
            let (a, b) = pair
                .split_once('-')
                .ok_or_else(|| input_error("Range not delimited by '-'."))?;

            let start = a.parse::<u64>()
                .map_err(|_| input_error(&format!("Invalid start number: {a}")))?;
            let end = b.parse::<u64>()
                .map_err(|_| input_error(&format!("Invalid ending number: {b}")))?;
            Ok((start, end))
        })
        .collect();
    items
}