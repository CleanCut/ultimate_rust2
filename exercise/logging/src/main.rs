// START IN lib.rs!!!

use frogger::Frog;

// You did 1-6 in lib.rs already, right?
//
// 7. Bring env_logger into scope. You might need to update Cargo.toml, first.

fn main() {
    // 8. Initialize env_logger using the init() function

    // 9. Run this program with `cargo run` and take a look at the default output.
    // - Now run it again with an explicit log level, like `RUST_LOG=info cargo run`
    // - ...and `RUST_LOG=trace cargo run`
    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();

    // Challenge: Go back to lib.rs and set the `target: ` argument for each logging call to be the
    // path to the function.  For example, `Frog::new`
}
