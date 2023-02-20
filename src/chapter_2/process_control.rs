pub fn if_let_demo() {
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

pub fn matches_demo() {
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