#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;
use std::ops::Add;

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

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary> (item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn some_func<T, u> (t: &T, u: &U) -> i32
//     where T: Display + Clone,
//     U: Clone + Debug
// {}

pub fn trait_demo() {
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

pub fn trait_demo2() {
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

pub fn trait_demo3() {
    let num_list = vec![1,2,3,4];
    let result = largest(&num_list);
    println!("The largest num is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd'];
    let result = largest(&char_list);
    println!("The largest num is {}", result);
}


#[derive(Debug)]
struct PointTemp<T> {
    x: T,
    y: T
}

// The T in impl<> is different with the one in PointTemp<>.
// The Output in impl<> corresponds to T, whereas the Output in the fn body corresponds to Point<T>.
impl<T: Add<T, Output = T>> Add for PointTemp<T> {
    type Output = PointTemp<T>;

    fn add(self, rhs: PointTemp<T>) -> PointTemp<T> {
        PointTemp {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

// The T in add<> corresponds to PointTemp<T> defined above.
fn add<T: Add<T, Output = T>> (a:T, b:T) -> T {
    a + b
}

pub fn trait_demo4() {
    let p1 = PointTemp{x: 1.1f32, y: 1.1f32};
    let p2 = PointTemp{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = PointTemp{x: 1i32, y: 1i32};
    let p4 = PointTemp{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}


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

pub fn trait_demo5() {
    let file = File::new("Rust Intro");
    println!("{}", file);
    println!("{:?}", file);
}

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

pub fn trait_object_demo() {
    let a = 1.1f64;
    let b = 3u8;
    draw1(Box::new(a));
    draw1(Box::new(b));

    draw2(&a);
    draw2(&b);
    
}


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

pub fn trait_with_default_generic_type_args_demo() {
    assert_eq!(Point {x: 1, y: 2} + Point {x: 2, y: 1}, Point {x: 3, y: 3});
    assert_eq!(Millimeter {0: 100} + Meter {0: 2}, Millimeter {0: 2100});
}

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

pub fn same_name_method_demo() {
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

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

pub fn trait_constraint_of_trait_demo() {
    let point = Point {x: 1, y: 3};
    point.outline_print();
}
