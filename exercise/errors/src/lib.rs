// 1. Create error type(s) representing the following three conditions:
// - An orca is hungry (Hungry)
// - An orca is too young (TooYoung)
// - The orca's name is too long (LongName)
//
// As a reminder, here are the 5 Guidelines for creating an error type
// (1) Use an `enum` for your error type
// (2) Your error conditions should be enum variants grouped in as few enums as makes sense
// (3) Don't expose error types other than your own to users
// (4) Make your enum non-exhaustive
// (5) Implement the Debug, Display, and Error traits
// (5b) You can use thiserror's `Error` macro to derive Display and Error.h
//
// Once you have completed the error type correctly, you should be able to run `cargo build --lib`
// without any errors.

// pub enum OrcaError...

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OrcaError {
    #[error("I'm too hungry to do that.")]
    Hungry,
    #[error("I'm too young to do that.")]
    TooYoung,
    #[error("I would, but my name is just so long!")]
    LongName,
}

pub struct Orca {
    pub name: String,
    pub age: u8,
    pub hungry: bool,
}

impl Orca {
    pub fn say_your_name(&self) -> Result<String, OrcaError> {
        if self.name.len() > 10 {
            Err(OrcaError::LongName)
        } else {
            Ok(format!("Hi, my name is {} and I'm an Orca!", self.name))
        }
    }
    pub fn flip(&self) -> Result<String, OrcaError> {
        if self.age < 4 {
            Err(OrcaError::TooYoung)
        } else {
            Ok(format!("Yippee, I'm doing a flip!"))
        }
    }
    pub fn shake_hands(&self) -> Result<String, OrcaError> {
        if self.hungry {
            Err(OrcaError::Hungry)
        } else {
            Ok(format!("Nice to meet you, let's shake hands!"))
        }
    }
}
