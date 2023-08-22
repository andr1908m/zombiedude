#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String};

use core::fmt::Write;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn my_string() -> String {
    let mut s = "hello".to_owned();
    s.write_str(" world");
    s.write_str("!");
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_string_test() {
        let v = my_string();
        assert_eq!(v, "hello world!");
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
