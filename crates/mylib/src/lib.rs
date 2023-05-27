#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use rand::seq::SliceRandom;

#[must_use]
pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn shuffle_array(nums: &mut [i32]) {
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 32), 42);
    }
}
