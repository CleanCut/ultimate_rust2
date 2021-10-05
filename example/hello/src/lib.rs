/// # Example
///
/// ```
/// # use hello::snuggle;
/// let bunnies = snuggle(5);
/// assert_eq!(bunnies, 40);
/// ```
pub fn snuggle(bunnies: u128) -> u128 {
    bunnies << 3
}

// The typical, multiplication approach
//
// pub fn snuggle(bunnies: u128) -> u128 {
//     bunnies * 8
// }

// The loop approach
//
// pub fn snuggle(bunnies: u128) -> u128 {
//     let mut result = 0;
//     for _ in 0..8 {
//         result += bunnies
//     }
//     result
// }

#[cfg(test)]
mod test {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn snuggling_bunnies_multiply() {
        assert_eq!(snuggle(2), 16);
    }

    #[should_panic]
    #[test]
    fn scared_bunny() {
        panic!("Hop hoppity hop!");
    }

    #[test]
    fn bunny_result() -> Result<(), ParseIntError> {
        let num_bunnies: u64 = "4".parse()?;
        assert_eq!(num_bunnies, 4);
        Ok(())
    }
}
