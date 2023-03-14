#![allow(dead_code)]
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn ownership_demo() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);
    
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

pub fn mut_demo() {
    // let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world");
}

pub fn mut_reference_demo() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
    // println!("{}, {}", r1, r2);
}