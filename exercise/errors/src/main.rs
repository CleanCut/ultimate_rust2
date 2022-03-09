// START IN lib.rs!

use aquarium::Dolphin;
// Silence some warnings so they don't distract from the exercise.
#[allow(clippy::vec_init_then_push)]

// (You already did #1 in lib.rs, right?)
//
// 2a. Uncomment and finish the play_time function below
// - Bring anyhow::Result into scope with a `use` statement
// - Have the play_time function return a `Result<Vec<String>>`. The vector of Strings will
//   represent successful outcomes of various dolphin tricks.

// fn play_time(dolphin: &Dolphin) -> ... {
//     let mut responses = vec![];
//     // 2b. Call the .say_your_name() method on `dolphin`, use `?` to unwrap the value, and push
//     // the value onto the `responses` vector.
//     //
//     // let response = ...    // this can be done with an intermediate variable...
//     // responses.push( ... ) // ...or all on one line. Either way is fine!
//     //
//     // 2c. Do the same thing as #2b for the .flip() method
//     //
//     // 2d. Do the same thing as #2b for the .shake_hands() method
//
//     Ok(responses)
// }

fn main() {
    let dolphins = vec![
        Dolphin {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Dolphin {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Dolphin {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Dolphin {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];
    for dolphin in &dolphins {
        // Challenge: Change main() so that it returns a Result, and instead of handling the error
        // that play_time returns, use the try (?) operator to only handle the success condition.
        // 
        // If done correctly, the output of the program will become much shorter. Since play_time
        // returns an Err variant the first time it is called, the try operator will return it from
        // main(), which will end the program at the first error. anyhow's Result will take care of
        // formatting the error output for us.
        match play_time(dolphin) {
            Ok(responses) => {
                println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
                for response in responses {
                    println!("  {}", response);
                }
            }
            Err(e) => println!("{} can't perform today: {}", dolphin.name, e.to_string()),
        }
    }
}
