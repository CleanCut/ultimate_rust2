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
    let puzzle = match get_puzzle("puzzle.dat").context("Couldn't get the first puzzle") {
        Ok(p) => p,
        Err(_) => Puzzle::new(),
    };
    info!("Playing puzzle: {}", puzzle.name);
    Ok(())
}
