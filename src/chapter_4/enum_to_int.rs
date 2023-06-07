#![allow(unused)]

// error
// pub enum MyEnum {
//     A = 1,
//     B,
//     C
// }

// pub fn enum_to_int_error_demo() {
//     let x = MyEnum::C as i32;
//     match x {
//         MyEnum::A => {},
//         MyEnum::B => {},
//         MyEnum::C => {},
//         _ => {},
//     }
// }

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum MyEnum {
    A = 1,
    B,
    C
}

pub fn enum_to_int_demo() {
    let x = MyEnum::C as i32;

    match FromPrimitive::from_i32(x) {
        Some(MyEnum::A) => println!("Got A"),
        Some(MyEnum::B) => println!("Got B"),
        Some(MyEnum::C) => println!("Got C"),
        None            => println!("Could not convert {}.", x),
    }
}

