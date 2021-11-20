use log::{error, info};
use std::fs::File;
use thiserror::Error;

/// Number of pieces in the puzzle
///
/// # History
///
/// This is a separate paragraph.
/// - Clickable link: [PUZZLE_PIECES]
/// - We tried `7`, but this is better
pub const PUZZLE_PIECES: u32 = 42;

/// This is a Puzzle!
#[derive(Clone, Debug)]
pub struct Puzzle {
    /// Number of pieces
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}

impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        let puzzle = Default::default();
        info!("Created a puzzle with new(): {:?}", puzzle);
        puzzle
    }
    /// Load a puzzle from a file
    pub fn from_file(_fh: File) -> Result<Self, PuzzleError> {
        println!("HERE");
        error!("This file is missing a piece!");
        Err(PuzzleError::MissingPiece)
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: PUZZLE_PIECES,
            name: "Forest Lake".to_string(),
        }
    }
}

impl PartialEq for Puzzle {
    fn eq(self: &Puzzle, other: &Puzzle) -> bool {
        (self.num_pieces == other.num_pieces)
            && (self.name.to_lowercase() == other.name.to_lowercase())
    }
}

impl From<&Puzzle> for String {
    fn from(puzzle: &Puzzle) -> Self {
        puzzle.name.clone()
    }
}

pub fn show<T: Into<String>>(s: T) {
    println!("{}", s.into());
}

pub fn blah() {
    let puzzle = Puzzle::default();
    show(&puzzle);
    // puzzle is still available!
}

#[derive(Clone, Copy)]
pub enum PuzzleType {
    Jigsaw,
}

// struct Vehicle;
// enum TransformerError {
//     Whatever,
// }

// struct Transformer;
// impl Transformer {
//     pub fn new() -> Self {
//         Self {}
//     }
//     pub fn stand(self) -> Result<Transformer, TransformerError> {
//         Ok(self)
//     }
//     pub fn transform(self) -> Result<Transformer, TransformerError> {
//         Ok(self)
//     }
//     pub fn rollout(self) -> Result<Transformer, TransformerError> {
//         Ok(self)
//     }
//     pub fn chase(self) -> Result<Transformer, TransformerError> {
//         Ok(self)
//     }
// }

// pub fn autobots_rollout() -> Result<Vehicle, TransformerError> {
//     let optimus = Transformer::new();
//     try!(try!(try!(try!(optimus.stand()).transform()).rollout()).chase())
//     // optimus.stand()?.transform()?.rollout()?.chase()?
// }

//     optimus.stand()?.transform()?.rollout()?.chase()?

//     let stand = match optimus.stand() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     let transform = match stand.transform() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     let rollout = match transform.rollout() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     match transform.chase() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };

//     match {
//         match {
//             match {
//                 match optimus.stand() {
//                     Ok(x) => x,
//                     Err(e) => return Err(e),
//                 }
//             }
//             .transform()
//             {
//                 Ok(x) => x,
//                 Err(e) => return Err(e),
//             }
//         }
//         .rollout()
//         {
//             Ok(x) => x,
//             Err(e) => return Err(e),
//         }
//     }
//     .chase()
//     {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     }
// }

// #[derive(Debug)] // #5: Debug + Display + Error
// #[non_exhaustive] // #4: Non-Exhaustive
// pub enum PuzzleError {
//     // #1: enum
//     WontFit(u16), // #2: Group Errors
//     MissingPiece, // #3: Only YOUR Errors
// }

// #5: Debug + Display + Error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPiece,
}

// #5: Debug + Display + Error
// impl Display for PuzzleError {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         use PuzzleError::*;
//         match self {
//             MissingPiece => write!(f, "Missing a piece"),
//             WontFit(n) => write!(f, "Piece {} doesn't fit!", n),
//         }
//     }
// }

// #5: Debug + Display + Error
// impl Error for PuzzleError {}
