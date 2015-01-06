#![feature(phase)]

#[phase(plugin, link)]
extern crate macros;

// This makes it work.
// #[cfg(good1)]
use macros::stuff::Thing;

// Horrible things here.
#[cfg(bad1)]
pub struct Thing {
    pub nonelt: u32,
}

fn main() {
    println!("inner: {}", macros::stuff::test());
    println!("outer: {}", test());

    {
        use macros::stuff::blop;
        println!("b1: {}", see_blop!());
    }

    {
        fn blop() -> &'static str {
            "outer blop"
        }
        println!("b2: {}", see_blop!());
    }
}

fn test() -> macros::stuff::Thing {
    thing!(8u8)
}
