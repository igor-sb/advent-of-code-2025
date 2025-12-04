use std::io;
use day01::{part1, part2, dial::Dial};

fn main() -> io::Result<()> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");

    let input_file = format!("{cargo_root}/data/input.txt");
    let mut dial = Dial { digit: 50 };
    let n_zeros = part1(&input_file, &mut dial)?;
    println!("Number of times dial pointed at zero: {n_zeros}");

    dial.digit = 50;
    let small_input_file = format!("{cargo_root}/data/input.txt");
    let n_zeros = part2(&small_input_file, &mut dial)?;
    println!("Number of times dial crossed zero: {n_zeros}");
    Ok(())
}