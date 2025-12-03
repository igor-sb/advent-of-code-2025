use std::io;

pub mod dial;
pub mod direction_iter;

use dial::{Dial, DIAL_DIGITS};

use crate::{dial::Direction, direction_iter::read_dial_directions};

pub fn part1(dial_directions: &str, dial: &mut Dial) -> Result<u16, io::Error> {
    let mut n_dial_at_zero: u16 = 0;
    for dial_direction in read_dial_directions(dial_directions)? {
        let (direction, rotation) = dial_direction?;
        dial.rotate(direction, rotation);
        if dial.digit == 0 {
            n_dial_at_zero += 1;
        }
    }
    Ok(n_dial_at_zero)
}

pub fn part2(dial_directions: &str, dial: &mut Dial) -> Result<u16, io::Error> {
    let mut n_dial_cross_zero: u16 = 0;
    for dial_direction in read_dial_directions(dial_directions)? {
        let (direction, rotation) = dial_direction?;
        n_dial_cross_zero += match direction {
            Direction::ClockWise => (dial.digit + rotation) / DIAL_DIGITS,
            Direction::CounterClockWise => {
                let mut n_full_circles = rotation / DIAL_DIGITS;
                if (rotation % DIAL_DIGITS >= dial.digit) && (dial.digit > 0) {
                    n_full_circles += 1;
                }
                n_full_circles
            },
        };
        dial.rotate(direction, rotation);
    }
    Ok(n_dial_cross_zero)
}