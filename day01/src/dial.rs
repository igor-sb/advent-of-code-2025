use core::fmt;
use std::convert::TryFrom;

pub static DIAL_DIGITS: u16 = 100;

#[derive(Debug)]
pub enum Direction {
    CounterClockWise,
    ClockWise,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::CounterClockWise),
            'R' => Ok(Direction::ClockWise),
            _ => Err("Expected 'L' or 'R'."),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}



pub struct Dial {
    pub digit: u16,
}

impl Dial {
    fn rotate_cw(&mut self, rotation: u16) {
        self.digit = (self.digit + rotation) % DIAL_DIGITS
    }
    fn rotate_ccw(&mut self, rotation: u16) {
        let number_remain = rotation % DIAL_DIGITS;
        if number_remain <= self.digit {
            self.digit -= number_remain;
        } else {
            self.digit += DIAL_DIGITS - number_remain;
        }
    }
    pub fn rotate(&mut self, dir: Direction, rotation: u16) {
        match dir {
            Direction::CounterClockWise => self.rotate_ccw(rotation),
            Direction::ClockWise => self.rotate_cw(rotation),
        }
    }
}
