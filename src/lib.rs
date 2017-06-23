

#![feature(test)]
extern crate rayon;
extern crate test;
use test::Bencher;

pub mod calls;
pub mod structs;
pub mod free_fns;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
