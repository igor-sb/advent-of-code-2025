use std::fs::File;
use std::io::{self, BufRead};
use crate::dial::Direction;

fn input_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

fn parse_dial_direction_string(
    dial_direction_string: &str,
) -> io::Result<(Direction, u16)> {
    if dial_direction_string.len() < 2 {
        return Err(input_error("Dial direction string is too short!"));
    }

    let direction_chr = dial_direction_string.as_bytes()[0] as char;
    let direction = Direction::try_from(direction_chr)
        .map_err(|e| input_error(e))?;
    let number: u16 = dial_direction_string[1..]
        .parse()
        .map_err(|e| input_error(&format!("Invalid number: {e}")))?;
    Ok((direction, number))
}

pub fn read_dial_directions(
    path: &str,
) -> io::Result<impl Iterator<Item = io::Result<(Direction, u16)>>> {
    let file = File::open(path)
        .map_err(|_| input_error(&format!("Failed to open file: {path}")))?;
    let reader = io::BufReader::new(file);

    Ok(
        reader.lines().map(|line_result| {
            line_result.and_then(|line| parse_dial_direction_string(&line))
        })
    )
}