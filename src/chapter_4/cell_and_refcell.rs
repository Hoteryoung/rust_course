#![allow(unused)]

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub fn cell_demo_1() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;

    x.set(2);
    y.set(3);
    z.set(4);

    println!("x: {}, y: {}, z: {}", x.get(), y.get(), z.get());
}

pub fn refcell_demo_1() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    // will panic during runtime, cause it violates the rules.
    let s2 = s.borrow_mut();
    println!("s1: {}, s2: {}", s1, s2);
}

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MessageQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MessageQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
        println!("{:?}", self.msg_cache);
    }
}

pub fn interior_mutability_demo() {
    let mq = MessageQueue{
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello".to_string());
}

pub fn rc_refcell_demo() {
    let s = Rc::new(RefCell::new("hahaha".to_string()));
    let s1 = s.clone();
    let s2 = s.clone();

    println!("{:?}\n{:?}\n{:?}\n", s, s1, s2);
    (*s).borrow_mut().push_str(", hehehe");
    println!("{:?}\n{:?}\n{:?}\n", s, s1, s2);
}