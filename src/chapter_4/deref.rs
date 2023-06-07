#![allow(unused)]

use std::ops:: {Deref, DerefMut};

fn display(s: &str) {
    println!("{}", s);
}

pub fn deref_demo_1() {
    let s= String::from("hello");
    let s_ = &s;
    display(s_);
}

pub fn deref_demo_2() {
    let s= Box::new(String::from("hello"));
    let s_ = &s;
    display(s_);
}

struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

pub fn deref_demo_3() {
    let f = &&Foo;

    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

pub struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox {v: x}
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    // No need to write following line.
    // type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

pub fn deref_demo_4() {
    let mut s = MyBox::new(String::from("hello, "));
    display_mut(&mut s);
    // can deref a &mut x into &x.
    display(&mut s);
}

fn display_mut(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}