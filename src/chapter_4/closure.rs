#![allow(unused)]

use std::thread;

pub struct Cache<T> 
where T: Fn(u32) -> u32
{
    value: Option<u32>,
    query: T
}

impl<T> Cache<T>
where T: Fn(u32) -> u32
{
    pub fn new(query: T) -> Cache<T>{
        Cache {
            value: None,
            query
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v)  => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn closure_demo() {
    let closure = |x| x;
    let mut cache = Cache::new(closure);
    let value = cache.value(50);
    println!("The value is {}", value);
}

fn fn_once<F>(func: F) 
where F: FnOnce(usize) -> bool
{
    println!("{}", func(3));
    // Error
    // println!("{}", func(4));
}

fn fn_once_with_copy<F>(func: F) 
where F: FnOnce(usize) -> bool + Copy
{
    println!("{}", func(3));
    // Not error
    println!("{}", func(4));
}

pub fn fn_once_demo() {
    let x = vec![1,2,3];
    // Note that fn_once and fn_once_with_copy can run together.
    // This means that x is actually not moved. 
    // But if we force move of x, as the next line does, then these two fns can not run together.
    // fn_once(move |z|{z == x.len()});
    // This experiment demonstrates that mark as FnOnce doesn't mean that it only implements FnOnce.
    fn_once( |z|{z == x.len()});
    fn_once_with_copy(|z|{z == x.len()});
}

pub fn move_demo () {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {println!("{:?}", v);});
    handle.join().unwrap();
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
}

pub fn fn_mut_demo() {
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    // Not error
    // let mut update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{}", s);
}

pub fn return_closure(x: i32) -> Box<dyn Fn(i32) -> i32>{
    let num = 5;
    if x > 1 {
        // Because the closure was returned, the num should be moved.
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

pub fn return_closure_demo() {
    let f = return_closure(3);
    let result = f(3);
    println!("{}", result);
}