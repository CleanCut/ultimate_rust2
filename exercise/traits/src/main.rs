pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

fn main() {
    // 1. The code below doesn't work because Cake doesn't implement Debug.
    // - Derive the Debug trait for the Cake enum above so this code will work.

    let cake = Cake::Spice;
    admire_cake(cake);

    // 2. Uncomment the code below. It doesn't work since `cake` was moved into the admire_cake()
    // function.
    // - Derive the Copy trait for the Cake enum so that `cake` gets copied into the
    //   admire_cake() function instead of moved.
    // - Hint: You may need to implement an additional trait in order to be able to implement Copy.

    // match cake {
    //     Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
    //     Cake::MapleBacon => println!("Dreams do come true!"),
    //     Cake::Spice => println!("Great, let's spice it up!"),
    // }

    // 3. Uncomment the println below
    // - Derive Debug for the Party struct above
    // - Manually implement the Default trait for the Party struct above. The default value you
    //   should return from your trait implementation should be:
    //
    //     Party {
    //         at_restaurant: true,
    //         num_people: 8,
    //         cake: Cake::Chocolate,
    //     }
    //
    // Hint: If you get stuck, there is an example at
    // https://doc.rust-lang.org/std/default/trait.Default.html#how-can-i-implement-default

    // println!("The default Party is\n{:#?}", Party::default());

    // 4. You prefer Maple Bacon cake. Use "struct update syntax" to create a Dessert with `cake`
    // set to `Cake::MapleBacon`, but the rest of the values are default.
    //
    // Hint: The trick to struct update syntax is specifying the value(s) you want to customize and
    // then ending the struct with `..Default::default()` -- but no comma after that!

    // let party = Party {
    //     ...
    // };
    // println!("Yes! My party has my favorite {:?} cake!", party.cake);

    // 5. Parties are "equal" if they have the same cake.
    // - Derive the PartialEq trait for the Cake enum so Cakes can be compared.
    // - Manually implement the PartialEq trait for Party. If different parties have the same cake,
    // then they are equal.
    // - Uncomment and run the code below.

    // let other_party = Party {
    //     at_restaurant: false,
    //     num_people: 235,
    //     cake: Cake::MapleBacon,
    // };
    // if party == other_party {
    //     println!("Your party is just like mine!");
    // }

    // Challenge: You would like to be able to pass a Party struct into the smell_cake() function
    // which takes a type T which implements the Into<Cake> trait.
    // - Uncomment the code below AND the smell_cake() function at the bottom of this file
    // - Implement From<Party> for Cake so that the code below will work.

    // smell_cake(party);
}

pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! 🎂", cake);
}

// pub fn smell_cake<T: Into<Cake>>(something: T) {
//     println!("Hmm...something smells like a {:?} cake!", something.into());
// }
