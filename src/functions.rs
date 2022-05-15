fn main() {
    println!("{}", is_even(3));
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //this is the return value of a function ( dont write a semicolon at the end)
}
