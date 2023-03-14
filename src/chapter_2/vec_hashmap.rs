#![allow(dead_code)]
pub fn vec_demo() {
    let v: Vec<i32> = Vec::new();
    let a = v.get(0);
    println!("{}", a == None);

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    for i in &mut v2 {
        *i *= 10;
        println!("{}", i);
    }

    let v3 = vec![1,2,3];
    for i in &v3 {
        println!("{}", i);
    }

}

// start of HashMap

use std::collections::HashMap;

pub fn hashmap_demo() {
    let mut my_gems = HashMap::new();
    my_gems.insert("blue", 1);
    my_gems.insert("red", 2);
    my_gems.get("green");
    let old = my_gems.insert("blue", 3);
    let red = my_gems["red"];
    let get_red = my_gems.get("red");
    println!("The old value of blue is {:?}", old);
    println!("{:?}", red);
    println!("{:?}", get_red);
    println!("{:?}", my_gems);

    let teams = vec![
        ("China".to_string(), 100),
        ("America".to_string(), 80),
        ("Japan".to_string(), 10),
    ];
    let mut teams_map: HashMap<_,_> = teams.into_iter().collect();
    let score = teams_map.get(&"China".to_string());
    if let Some(x) = score {
        println!("{}", x);
    }
    for (k,v) in &teams_map {
        println!("{}: {}", k, v);
    }
    println!("{}", "**************************");
    teams_map.entry("America".to_string()).or_insert(70);
    teams_map.entry("France".to_string()).or_insert(70);
    for (k,v) in &teams_map {
        println!("{}: {}", k, v);
    }
}

pub fn update_hashmap_demo() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", *count);
        println!("{}", count);
    }
    println!("{:?}", map);
}
