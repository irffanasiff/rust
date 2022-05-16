fn main() {
    let str: &str = "hello world";
    let mut str2: String = String::from("Hello World");
    println!(
        "str - {} len - {} | str2 - {} len - {}",
        str,
        str.len(),
        str2,
        str2.len()
    );
    let slice = &str2[..6];
    println!("slice - {}, len {}", slice, slice.len());
    str2.push('1');
    str2.push_str("2");
}
