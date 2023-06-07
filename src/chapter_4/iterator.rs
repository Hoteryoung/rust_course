#![allow(unused)]

pub fn iter_demo() {
    let values = vec![1,2,3];
    let values_into_iter = values.into_iter();
    for v in values_into_iter {
        println!("{}", v);
    }
    // This line will cause error, because values is moved due to the into_iter call.
    // println!("{:?}", values);
    println!("----------------------");

    let values = vec![1,2,3];
    let values_iter = values.iter();
    for v in values_iter {
        println!("{}", v);
    }
    println!("{:?}", values);
    println!("----------------------");

    let mut values = vec![1,2,3];
    let mut values_iter_mut = values.iter_mut();
    for v in values_iter_mut {
        *v += 1;
    }
    println!("{:?}", values);
}

pub fn sum_demo() {
    let values = vec![1,2,3];

    let values_iter = values.iter();

    let total: i32 = values_iter.sum();

    println!("{}", total);
    println!("{:?}", values);
    // values_iter was moved due to the sum call.
    // println!("{:?}", values_iter);
}

pub fn map_filter_collect_demo() {
    let values = vec![1,2,3];
    // Both are correct. I should use the second one.
    // let collected_values: Vec<_> = values.iter().map(|x| x + 1).collect();
    // Type must be supplied.
    let collected_values: Vec<_> = values.iter().map(|x| *x + 1).collect();
    println!("{:?}", values);
    println!("{:?}", collected_values);

    let filtered_values: Vec<_> = values.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", values);
    println!("{:?}", filtered_values);

    let collected_values: Vec<_> = values.into_iter().map(|x| x + 1).collect();
    println!("{:?}", collected_values);
}

use::std::collections::HashMap;

pub fn zip_collect_demo() {
    let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];

    let kv: HashMap<_,_> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", kv);
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn iterator_demo() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}