#![allow(unused)]
#![warn(clippy::has_loop)]

fn main() {
    let mut i = 4;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}
