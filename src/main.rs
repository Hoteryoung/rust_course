/*
fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界， 你好！";
    let english = "World, hello!";
    let regions = [southern_germany, chinese, english];
    // for region in regions.iter() {
    for region in regions {
        println!("{}", &region);
    }
}

fn penguin() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn basic_concept() {
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

fn assert_try() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    // assert_eq!(a, !b);
    assert_eq!(a, b);
} 

fn int_overflow() {
    let a: u8 = u8::MAX;
    let b = a.wrapping_add(20);
    let c = a.checked_add(20);
    let (d, e) = a.overflowing_add(20);
    println!("b: {}", b);
    println!("c: {:?}", c);
    println!("d: {}, e: {}", d, e);
    println!("c: {:?}", c);
}

fn range_demo() {
    // for i in 1..=5 {
    // for i in 1..=5 {
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

use num::complex::Complex;

fn complex_demo() {
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

fn char_demo() {
    let c = 'c';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}.{}.{}.{}", c, z, g, heart_eyed_cat);
}

fn ownership_demo() {
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

fn mut_demo() {
    // let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world");
}

fn mut_reference_demo() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
    // println!("{}, {}", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn string_slice_demo() {
    // let mut s = String::from("hello, world");
    let s = String::from("hello, world");
    let _word = first_word(&s);

    // s.clear();

    println!("{}", _word);
}

fn first_word(s: &String) ->&str {
    &s[..5]
}
*/

/* 
fn array_slice_demo() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    assert_eq!(slice, &a[1..3]);
    assert_eq!(*slice, [2, 3]);
    assert_eq!(slice, [2, 3]);
    
    let b = [2, 3, 4];
    // The next also succeeds.
    assert_eq!(slice, &b[0..2]);
}

fn string_str_demo() {
    let s_1 = String::from("hello");
    let s_2 = "hello".to_string();

    say_hello(&s_1);
    say_hello(&s_2[..]);
    say_hello(s_1.as_str());
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn operate_string() {
    let mut s = String::from("Hello ");
    s.push('r');
    s.push_str("ust!");
    s.insert(5, ',');
    s.insert_str(6, " I like");
    println!("{}", s);
    let mut sr = s.replace("rust", "RUST");
    println!("{}", sr);
    sr = s.replacen("like", "love" , 1);
    println!("{}", sr);
    s.replace_range(0..5, "Hola");
    println!("{}", s);
    let sp = s.pop();
    println!("{:?}", sp);
    let srm = s.remove(0);
    println!("{}", srm);
    s.clear();
    println!("{}", s);

    let string_append = String::from("Hello ");
    let string_rust = "rust".to_string();
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("{}", result);

    let sf = format!("{} {}", string_rust, result);
    println!("{}", sf);

    for c in "中国人".chars() {
        println!("{}", c);
    }
}

fn tuple_demo() {
    let tup: (String, f64, u8) = (String::from("hello"), 6.4, 1);
    let (_x,y,_z) = tup;
    println!("{}", y);
    // note that tuple is partially moved.
    println!("{}", tup.2);
} 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_demo() {
    let mut user1 = User {
        email: String::from("hoteryoung@sina.com"),
        username: String::from("hoteryoung"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@sina.com");

    let user2 = build_user("thethirdemail@sina.com".to_string(), "hoteryoung".to_string());

    let user3 = User {
        email: "theforthemail@sina.com".to_string(),
        ..user2
    };

    println!("{}", user3.active);
    println!("{}", user3.username);
    println!("{}", user3.sign_in_count);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let color = Color(0,0,0);
    let point = Point(0,0,0);

    println!("{}", color.0);
    println!("{}", point.0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8)
}

fn enum_demo() {
    let p1 = PokerCard::Clubs(3);
    let p2 = PokerCard::Diamonds(6);
    let p3 = PokerCard::Spades(9);
    let p4 = PokerCard::Hearts(11);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{:?}", p4);

    let x = Some(5);
    println!("{:?}", x);
    let x_plus_one = plus_one(x);
    println!("{:?}", x_plus_one);
    let none = plus_one(None);
    println!("{:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn array_demo() {
    let one = [1,2,3]; 
    let two:[u8; 3] = [1,2,3];

    let blank1 = [0;3];
    let blank2: [u8; 3] = [0;3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        println!("{:?}", a);
        for n in a {
            println!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\tsum({:?}) = {}", a, sum);
    }
}

fn if_let_demo() {
    let a = Some(5);
    if let Some(i) = a {
        println!("{:?}", i);
    }
    if let Some(5) = a {
        println!("{:?}", a);
    }
}
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar
}

fn matches_demo() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let vf = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    for i in vf {
        println!("{:?}", i);
    }
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(5);
    assert!(matches!(bar, Some(a) if a > 2));
}

// start of trait_demo

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary> (item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn some_func<T, u> (t: &T, u: &U) -> i32
//     where T: Display + Clone,
//     U: Clone + Debug
// {}

fn trait_demo() {
    let post = Post {
        title: "Rust Intro".to_string(),
        author: "Hunter".to_string(),
        content: "Rust is great!".to_string()
    };

    let weibo = Weibo {
        username: "Hunter".to_string(),
        content: "Rust is awesome!".to_string()
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    notify(&weibo);
    notify2(&weibo);
}

// start of trait_demo2

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The larger number is x = {}", self.x);
        } else {
            println!("The larger number is y = {}", self.y);
        }
    }
}

fn trait_demo2() {
    let p= Pair{
        x: 3,
        y: 4
    };
    p.cmp_display();
    let p2 = Pair::new(5,6);
    p2.cmp_display();
}

//start of trait_demo3

fn largest<T: PartialOrd + Copy> (list: &[T]) ->T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn trait_demo3() {
    let num_list = vec![1,2,3,4];
    let result = largest(&num_list);
    println!("The largest num is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd'];
    let result = largest(&char_list);
    println!("The largest num is {}", result);
}

//start of trait_demo4

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

// The T in impl<> is different with the one in Point<>.
// The Output in impl<> corresponds to T, whereas the Output in the fn body corresponds to Point<T>.
impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

// The T in add<> corresponds to Point<T> defined above.
fn add<T: Add<T, Output = T>> (a:T, b:T) -> T {
    a + b
}

fn trait_demo4() {
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}

//start of trait_demo5

#![allow(dead_code)]
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self is a pointer
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: name.to_string(),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn trait_demo5() {
    let file = File::new("Rust Intro");
    println!("{}", file);
    println!("{:?}", file);
}

//start of trait_object_demo

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    println!("{}", x.draw());
}

fn draw2(x: &dyn Draw) {
    println!("{}", x.draw());
}

fn trait_object_demo() {
    let a = 1.1f64;
    let b = 3u8;
    draw1(Box::new(a));
    draw1(Box::new(b));

    draw2(&a);
    draw2(&b);
    
}

//start of trait_with_default_generic_type_args_demo

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[derive(Debug, PartialEq)]
struct Meter(u32);
#[derive(Debug, PartialEq)]
struct Millimeter(u32);

impl Add<Meter> for Millimeter {
    type Output = Millimeter;
    fn add(self, rhs: Meter) -> Millimeter {
        Millimeter {
            0: self.0 + rhs.0 * 1000
        }
    }
}

fn trait_with_default_generic_type_args_demo() {
    assert_eq!(Point {x: 1, y: 2} + Point {x: 2, y: 1}, Point {x: 3, y: 3});
    assert_eq!(Millimeter {0: 100} + Meter {0: 2}, Millimeter {0: 2100});
}

//start of same_method name demo

trait Wizard {
    fn fly(&self);
}
trait Pilot {
    fn fly(&self);
}

trait Animal {
    fn name();
}

struct Human;

impl Wizard for Human {
    fn fly(&self) {
        println!("Flying on a bloom!");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Flying in a plane!");
    }
}

impl Animal for Human {
    fn name() {
        println!("humankind!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Flying by waving hands?");
    }

    fn name() {
        println!("Hunter!");
    }
}

fn same_name_method_demo() {
    // The next two lines both work
    // let person = Human{};
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    Human::name();
    <Human as Animal>::name();
    <Human as Wizard>::fly(&person);
    <Human as Pilot>::fly(&person);
}
*/

//start of trait constraint of trait demo

use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32
}

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

fn trait_constraint_of_trait_demo() {
    let point = Point {x: 1, y: 3};
    point.outline_print();
}
fn main() {
    // greet_world();
    // penguin();
    // basic_concept();
    // assert_try();
    // int_overflow();
    // range_demo();
    // complex_demo();
    // char_demo();
    // ownership_demo();
    // mut_demo();
    // mut_reference_demo();
    // string_slice_demo();
    // array_slice_demo();
    // string_str_demo();
    // operate_string();
    // tuple_demo();
    // struct_demo();
    // enum_demo();
    // array_demo();
    // if_let_demo();
    // matches_demo();
    // trait_demo();
    // trait_demo2();
    // trait_demo3();
    // trait_demo4();
    // trait_demo5();
    // trait_object_demo();
    // trait_with_default_generic_type_args_demo();
    // same_name_method_demo();
    trait_constraint_of_trait_demo();
}