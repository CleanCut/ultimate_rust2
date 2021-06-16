// START IN lib.rs!

use aquarium::Orca;

// (You already did #1 in lib.rs, right?)
// 2a. Uncomment and finish the play_time function below
// - Bring anyhow::Result into scope with a `use` statement
// - Have the function return a Result<Vec<String>>. The vector of Strings will represent successful
//   outcomes of various performances.

// fn play_time(orca: &Orca) -> ... {
//     let mut responses = vec![];
//     // 2b. There are three methods on the Orca struct:
//     // - .say_your_name()
//     // - .flip()
//     // - .shake_hands()
//     //
//     // For each of the three methods above:
//     // - Call the method on `orca`, using the `?` operator to unwrap the value / return the error
//     // - Push the unwrapped string onto the `responses` vector using the .push() method

//     // let response = ...    // this can be done with an intermediate variable...
//     // responses.push( ... ) // ...or all on one line. Either way is fine!

//     Ok(responses)
// }

fn main() {
    let orcas = vec![
        Orca {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Orca {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Orca {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Orca {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];
    for orca in &orcas {
        match play_time(orca) {
            Ok(responses) => {
                println!("{} did a FABULOUS PERFORMANCE!", orca.name);
                for response in responses {
                    println!("  {}", response);
                }
            }
            Err(e) => println!("{} can't perform today: {}", orca.name, e.to_string()),
        }
    }
}
