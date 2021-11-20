use anyhow::{Context, Result};
use log::info;
use puzzles::Puzzle;
use std::fs::File;

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    let fh = File::open(filename)
        .with_context(|| format!("couldn't open the puzzle file {}", filename))?;
    let puzzle = Puzzle::from_file(fh).context("couldn't convert data into a puzzle")?;
    Ok(puzzle)
}

fn main() -> Result<()> {
    env_logger::init();
    // This gets the absolute path to the puzzle.dat file in examples/puzzle_game no matter what
    // directory you are in when you run the `cargo run` command.
    let puzzle_file_path = &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "puzzle.dat");
    let puzzle = match get_puzzle(puzzle_file_path).context("Couldn't get the first puzzle") {
        Ok(p) => p,
        Err(_) => Puzzle::new(),
    };
    info!("Playing puzzle: {}", puzzle.name);
    Ok(())
}
