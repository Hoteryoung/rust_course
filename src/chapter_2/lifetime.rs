#![allow(dead_code)]
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetime_func() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}.", result);
}

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn announce_and_return_part_explicit_lifetime<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn announce_and_return_part_manual_lifetime<'b>(&'a self, announcement: &'b str) -> &'b str 
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// pub fn lifetime_struct() {
//     let excerpt;
//     {
//         let novel = String::from("My name is Hunter. Seven years ago ...");
//         let first_sentence = novel.split('.').next().expect("There is no '.'");
//         excerpt = ImportantExcerpt {
//             part: first_sentence,
//         };
//     }
//     println!("{}", excerpt.part);
// }

pub fn lifetime_struct() {
    let novel = String::from("My name is Hunter. Seven years ago ...");
    let first_sentence = novel.split('.').next().expect("There is no '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", excerpt.part);
    let part = excerpt.announce_and_return_part("implicit_lifetime");
    println!("{}", part);
    let part = excerpt.announce_and_return_part_explicit_lifetime("explicit_lifetime");
    println!("{}", part);
    let part = excerpt.announce_and_return_part_manual_lifetime("manual_lifetime");
    println!("{}", part);
}