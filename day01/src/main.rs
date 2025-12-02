use core::fmt;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {s}")),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}



struct Dial {
    digit: u16,
}

impl Dial {
    fn rotate_right(&mut self, number: u16) {
        self.digit = (self.digit + number) % 100
    }
    fn rotate_left(&mut self, number: u16) {
        if number <= self.digit {
            self.digit = self.digit - number;
        } else {
            self.digit = 100 - (number - self.digit);
        }
    }
    fn rotate(&mut self, direction: char, number: u16) {

    }
}

fn main() -> io::Result<()> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let input_file = File::open(
        format!("{cargo_root}/data/input.txt")
    ).expect("Missing input.txt file!");
    println!("Input file: {input_file:?}");

    let mut number_of_zeros: u16 = 0;
    let mut dial = Dial {
        digit: 50,
    };
    let reader = io::BufReader::new(input_file);
    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 { continue; }
        let direction = line.as_bytes()[0] as char;
        let number_str = &line[1..];
        let number: u16 = number_str.parse::<u16>().unwrap();
        println!("direction: {direction}, number: {number}");
        dial.rotate(direction, number);
        if dial.digit == 0 {
            number_of_zeros += 1;
        }
    }
    // let mut dial = Dial {
    //     digit: 99,
    // };
    // println!("Dial is at: {}", dial.digit);
    // dial.rotate_right(2);
    // println!("Dial is now at: {}", dial.digit);
    // dial.rotate_left(2);
    // println!("Dial is now at: {}", dial.digit);
    // dial.rotate_right(201);
    // println!("Dial is now at: {}", dial.digit);
    Ok(())
}