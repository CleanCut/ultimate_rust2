// Yes, yes, we know. It's an exercise, compiler, we want it that way!
#[allow(unused_mut)]

fn main() {
    // 1. Create a closure that returns the square of an integer (the number multiplied by itself),
    // and assign the closure to a variable called "square".

    // let square = ...
    //println!("5 squared is {}", square(5));

    // 2. Uncomment the code below.  Finish the .map() by passing it a closure which takes a tuple
    // of two integers, and returns a tuple with the first integer incremented by 1, and the second
    // integer left alone.  For example, (0, 1) should become (1, 1).

    // let pairs = vec![(0, 1), (2, 3), (4, 5)];
    // pairs
    //     .into_iter()
    //     .map( ... )
    //     .for_each(|t| println!("{:?}", t));

    // 3. Uncomment the code below. There is a mutable vector named `numbers`. Use an iterator over
    // mutable references to multiply each of the values in the numbers in vector by 3 without
    // consuming the vector.
    // Hint 1: You'll probably want to use .iter_mut()
    // Hint 2: `x` will be a mutable reference, so remember to dereference it wherever you use it

    // let mut numbers = vec![1, 2, 3, 4];
    // for x in ... {
    //     ... // square the value via the mutable reference x
    // }
    // println!("{:?}", numbers); // should print [3, 6, 9, 12]

    // 4. Uncomment the code below.  Take the vector of words and
    // - Convert the vector into an iterator with .into_iter()
    // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
    // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
    // - Use .collect() to put the transformed words back into a vector
    //
    // Hint: .to_uppercase() is a method on `str` which returns a String

    // let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    // let transformed...  // do the stuff here
    // println!("Transformed: {:?}", transformed);

    // Challenge: Both .iter() and .iter_mut() can be used via shorter "syntactic sugar" in a
    // for-loop definition. For example, instead of:
    //
    //     for x in vector.iter() { ... }
    //
    // you can do:
    //
    //     for x in &vector { ... }
    //
    // Can you figure out how to change .iter_mut() in #3 to the shorter, syntactic sugar form for
    // mutable references?
}
