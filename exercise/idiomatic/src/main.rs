// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

const pi:f32=3.14159265358979323846;
fn count_to_5()->i32{let mut foo =0;loop{if foo>pi as i32{if foo > 5{break;}}foo=foo+1;}return 5;}
fn main() {
println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_counting() {
assert_eq!(count_to_5() == 5, true);
}
}
