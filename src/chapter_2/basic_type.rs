pub fn basic_concept() {
    let a = 10;
    let b: i32 = 20; 
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("((a + b) + (c + d)) =  {}", e);
    c = 40_i32;
    let e = add(add(a, b), add(c, d));
    println!("((a + b) + (c + d)) =  {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

pub fn assert_try() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    // assert_eq!(a, !b);
    assert_eq!(a, b);
} 

pub fn int_overflow() {
    let a: u8 = u8::MAX;
    let b = a.wrapping_add(20);
    let c = a.checked_add(20);
    let (d, e) = a.overflowing_add(20);
    println!("b: {}", b);
    println!("c: {:?}", c);
    println!("d: {}, e: {}", d, e);
    println!("c: {:?}", c);
}

pub fn range_demo() {
    // for i in 1..=5 {
    // for i in 1..=5 {
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

use num::complex::Complex;

pub fn complex_demo() {
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

pub fn char_demo() {
    let c = 'c';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}.{}.{}.{}", c, z, g, heart_eyed_cat);
}