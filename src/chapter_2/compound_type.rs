pub fn string_slice_demo() {
    // let mut s = String::from("hello, world");
    let s = String::from("hello, world");
    let _word = first_word(&s);

    // s.clear();

    println!("{}", _word);
}

fn first_word(s: &String) ->&str {
    &s[..5]
}

pub fn array_slice_demo() {
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

pub fn string_str_demo() {
    let s_1 = String::from("hello");
    let s_2 = "hello".to_string();

    say_hello(&s_1);
    say_hello(&s_2[..]);
    say_hello(s_1.as_str());
}

fn say_hello(s: &str) {
    println!("{}", s);
}

pub fn operate_string() {
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

pub fn tuple_demo() {
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

pub fn struct_demo() {
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

pub fn enum_demo() {
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

pub fn array_demo() {
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