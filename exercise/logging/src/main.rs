// START IN lib.rs!!!

use frogger::Frog;

// You did #1-#6 in lib.rs already, right?
//
// 7. Update Cargo.toml to add the `env_logger` dependency

fn main() {
    // 8. Initialize env_logger using the init() function at the top level of the library

    // 9. Run this program with `cargo run` and take a look at the default output.
    // - Now run it again with an explicit log level, like `RUST_LOG=info cargo run`
    // - or `RUST_LOG=trace cargo run`
    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();
    skippy.sleep();

    // Challenge: Go back to lib.rs and set the `target: ` argument for each logging call to be the
    // path to the function.  For example, `Frog::new`
}
