#![allow(unused)]

pub fn reference_demo_1() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // x += 10;
    // let m = &mut x;
    println!("r1: {}, r2: {}", r1, r2);
}

pub fn reference_demo_2() {
    let mut x = 10;
    let m = &mut x;
    x += 10;
    println!("x: {}", x);
}

pub fn reference_demo_3() {
    let mut y = 20;
    let m1 = &y;
    let m2 = &y;
    let z = y;
    println!("m1: {}, m2: {}, z: {}", m1, m2, z);
}

pub fn reference_demo_4() {
    let mut y = 20;
    let m = &mut y;
    //Accessible only through the reference.
    // let z = y;
    println!("m: {}", m);
}

pub fn reference_demo_5() {
    let mut v = (107, 109);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    // v.1;
    println!("r1: {}", r1);
}