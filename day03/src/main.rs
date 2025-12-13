use std::error::Error;
use day03::part1;

fn main() -> Result<(), Box<dyn Error>> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let input_file = format!("{cargo_root}/data/input.txt");
    println!("Input file: {input_file}");
    part1(&input_file)?;
    Ok(())
}
