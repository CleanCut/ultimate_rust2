// 1. Create a DolphinError type representing the following three conditions:
// - Hungry - An dolphin is hungry
// - TooYoung - An dolphin is too young
// - LongName - The dolphin's name is too long and annoying to say
//
// As a reminder, here are the 5 Guidelines for creating an error type:
// (1) Use an `enum` for your error type
// (2) Your error conditions should be enum variants grouped in as few enums as makes sense
// (3) Don't expose error types other than your own (you don't have to do anything for this one)
// (4) Make your enum non-exhaustive
// (5) Implement the Debug, Display, and Error traits
// (5b) You can use thiserror's `Error` macro to derive Display and Error.h
//
// Once you have completed the error type correctly, you should be able to run `cargo build --lib`
// without any errors.

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DolphinError {
    #[error("I'm too hungry!")]
    Hungry,
    #[error("I'm too young to do that!")]
    TooYoung,
    #[error("I would, but my name is just soooo long!")]
    LongName,
}

pub struct Dolphin {
    pub name: String,
    pub age: u8,
    pub hungry: bool,
}

impl Dolphin {
    pub fn say_your_name(&self) -> Result<String, DolphinError> {
        if self.name.len() > 10 {
            Err(DolphinError::LongName)
        } else {
            Ok(format!("Hi, my name is {} and I'm a Dolphin!", self.name))
        }
    }
    pub fn flip(&self) -> Result<String, DolphinError> {
        if self.age < 4 {
            Err(DolphinError::TooYoung)
        } else {
            Ok(format!("Yippee, I'm doing a flip!"))
        }
    }
    pub fn shake_hands(&self) -> Result<String, DolphinError> {
        if self.hungry {
            Err(DolphinError::Hungry)
        } else {
            Ok(format!("Nice to meet you, let's shake hands!"))
        }
    }
}
