fn basics() {
    // unsigned integers
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10;

    // signed integers
    // u8, u16, u32, u64, u128
    let signed: i8 = -10;

    // float is used for decimals
    let float: f64 = 10.5;

    println!(
        "unsigned: {}, signed: {}, float: {}",
        unsigned, signed, float
    );

    // char can only be
    let letter = "c3425";
    let emoji = "\u{1F600}";

    println!("letter: {}, emoji: {}", letter, emoji);

    //boolean
    let is_true: bool = true;
    println!("is_true: {}", is_true);

    // arrays
    let arr: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let arr2: [i16; 5] = [100; 5];

    println!("arr: {:?}, arr2: {:?}", arr, arr2);
    println!("{}", arr2.len());

    let tuple: (u8, bool, f32) = (10, true, 10.5);
    let tuple2 = (3, 5);

    println!("first {}, second{}, third {}", tuple.0, tuple.1, tuple.2);

    let (a, b, c) = tuple;

    println!("a: {}, b: {}, c: {}", a, b, c);
}
