#![allow(dead_code)]
#![allow(unused)]

#[derive(Debug)]
pub struct Foo;

impl Foo {
    pub fn mutate_and_share(&mut self) -> &Self {
        &*self
    }

    pub fn immutate_and_share(&self) -> &Self {
        &*self
    }

    pub fn share(&self) {}
}

pub fn lifetime_demo() {
    // error
    // let mut foo = Foo;
    // let loan = foo.mutate_and_share();

    // No error
    let foo = Foo;
    let loan = foo.immutate_and_share();

    foo.share();
    println!("{:?}", loan);
}

pub fn hashmap_demo() {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where 
        K: Clone + Eq + Hash,
        V: Default,
    {
        // original failure:
        // match map.get_mut(&key) {
        //     Some(value) => value,
        //     None => {
        //         map.insert(key.clone(), V::default());
        //         map.get_mut(&key).unwrap()
        //     }
        // }

        // fix method 1: 
        // map.entry(key.clone()).or_insert(V::default())

        //fix method 2: The lifetime of immutable borrow ended at the match line.
        // match map.get(&key) {
        //     Some(_) => map.get_mut(&key).unwrap(),
        //     None => {
        //         map.insert(key.clone(), V::default());
        //         map.get_mut(&key).unwrap()
        //     }
        // }
        
        //fix method 3: Same reason as method 2.
        match map.get_mut(&key) {
            // Some(value) =>{} value,
            None => {
                map.insert(key.clone(), V::default());
                // map.get_mut(&key).unwrap()
            }
            _ => (),
        };
        return map.get_mut(&key).unwrap();
    }
}

pub fn closure_demo() {
    pub fn fn_elision(x: &i32) -> &i32 { x }
    // let closure_slision = |x: &i32| -> &i32 {x};
    let a = 3_i32;
    let b = &a;
    let c = fn_elision(b);

    let d;
    {
        let e = &a;
        d = fn_elision(e);
    }
    println!("{}", d);
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn reborrow_demo() {
    let mut point = Point{x: 0, y: 0};

    let r = &mut point;

    let rr = &*r;
    println!("{:?}", rr);

    r.move_to(3, 7);
    println!("{:?}", r);

}

pub struct Manager<'a> {
    text: &'a str
}

// Error
// pub struct Interface<'a> {
//     manager: &'a mut Manager<'a>
// }

// Correct
pub struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("Interface consumed");
    }
}

pub struct List<'a> {
    manager: Manager<'a>
}

impl<'a> List<'a> {
    // pub fn get_interface(&'a mut self) -> Interface
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> 
    where 'a: 'b 
    {
        Interface {
            manager: &mut self.manager
        }
    }
}

pub fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

pub fn last_lifetime_demo() {
    let mut list = List {
        manager: Manager {
            text: "hello world"
        }
    };
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");
    use_list(&list);
}

pub fn print_author(author: &'static str) {
    println!("{}", author);
}

pub fn static_demo() {
    let mark_twain = "Samuel Clemens";
    print_author(mark_twain);
}

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

pub fn get_memory_location() -> (usize, usize) {
    let string = "Hello world";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

pub fn get_string_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
    }
}

pub fn static_ref_demo() {
    let (pointer, length) = get_memory_location();
    let string = get_string_at_location(pointer, length);
    println!("The {} bytes at 0x{} stored: {}", length, pointer, string);
}