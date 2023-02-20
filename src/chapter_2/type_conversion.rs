pub fn type_cast_demo() {
    let a_0 = 254_i32 as u8;
    let a_1 = 255_i32 as u8;
    let a_2 = 256_i32 as u8;
    let a_3 = 257_i32 as u8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("a_0: {}\na_1: {}\na_2: {}\na_3: {}\nb: {}\nc: {}", a_0, a_1, a_2, a_3, b, c);

    let d = 500_i16;
    let d_: u8 = match d.try_into() {
        Ok(d_u8) => d_u8,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    println!("d_: {}", d_);
}
