use std::io;

use day02::{part1, part2};

fn main() -> Result<(), io::Error> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let input_file = format!("{cargo_root}/data/input.txt");
    println!("Input file: {input_file}");
    part1(&input_file)?;
    part2(&input_file)?;
    Ok(())
}
