#![allow(unused)]

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn newtype_demo() {
    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("{}", w);
}

type Meters = u32;
pub fn alias_demo() {
    let x = 1;
    let y: Meters = 2;
    println!("{}", x + y);
}


