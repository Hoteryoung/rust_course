#![allow(unused)]

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

pub fn rc_demo() {
    let gadget_owner = Rc::new(
        Owner{
            name: "gadget man".to_string()
        }
    );

    let gadget_1 = Gadget{
        id: 1,
        owner: Rc::clone(&gadget_owner)
    };
    
    let gadget_2 = Gadget{
        id: 2,
        owner: Rc::clone(&gadget_owner)
    };

    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget_1.id, gadget_1.owner.name);
    println!("Gadget {} owned by {}", gadget_2.id, gadget_2.owner.name);
}

pub fn arc_demo() {
    let s = Arc::new(String::from("multi thread wander"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(
            move || {println!("{}", s)}
        );
    }
}