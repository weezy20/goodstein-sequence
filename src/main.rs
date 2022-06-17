#![allow(unused)]
use goodstein_seq::{Base, Exponent};
use easy_io::InputReader;
fn main() {
    let mut i = InputReader::new();
    let mut n: u32 = i.next();
    let binary_n = format!("{n:b}").into_bytes();
    println!("You entered {binary_n:?}");
    println!("You entered {n:b}");
}
