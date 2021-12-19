pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    match (x, y, z) {
        (x, _, _) if x < 0 => 99,
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        (x, y, z) => x + y - z,
    }
}

pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

// 1. Use the `cfg` attribute to mark the `test` module below as a test module

mod test {
    // 2. Bring all the library items into scope with a `use` statement
    // Hint: It's okay to use `*` here.

    // 3. Write a test function that verifies the following condition using the `assert_eq!` or
    // `assert_ne!` macros
    // - sploosh(1, 2, 3) returns 4
    // - sploosh(5, 6, 7) does not return 4
    // - If you pass sploosh a negative number for the first argument, 99 is returned
    //
    // `cargo test` should run your tests and pass
    // Hint: Don't forget the `#[test]` attribute for your test function!

    // 4. Write a test function that verifies the following conditions using the `assert!` macro
    // - splish(100, 10) is negative
    // - splish(40, 20) is positive
    // - splish(9, 3) is 0
}

// 5. Create a `tests/` directory and an integration test file `tests/more_tests.rs`
// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
//
// `cargo test` should run your `more_tests.rs` file and pass

// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
