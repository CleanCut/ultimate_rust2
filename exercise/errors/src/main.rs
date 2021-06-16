// START IN lib.rs!

use aquarium::Dolphin;

// (You already did #1 in lib.rs, right?)
// 2a. Uncomment and finish the play_time function below
// - Bring anyhow::Result into scope with a `use` statement
// - Have the play_time function return a `Result<Vec<String>>`. The vector of Strings will
//   represent successful outcomes of various dolphin tricks.

// fn play_time(dolphin: &Dolphin) -> ... {
//     let mut responses = vec![];
//     // 2b. There are three methods on the Dolphin struct:
//     // - .say_your_name()
//     // - .flip()
//     // - .shake_hands()
//     //
//     // For each of the three methods above:
//     // - Call the method on `dolphin`, using the `?` operator to unwrap the value / return the error
//     // - Push the unwrapped string onto the `responses` vector using the .push() method

//     // let response = ...    // this can be done with an intermediate variable...
//     // responses.push( ... ) // ...or all on one line. Either way is fine!

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
