fn function() {
    let mut num = 5; //variable mutability
    num = 3;
    //   println!("num: {}", num);
    //   println!("{}", is_even(3));

    let arr = [0, 1, 2, 3]; // length is known at compile time
    let slice = &arr[1..3]; // [1,2] length is not fixed
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //this is the return value of a function ( dont write a semicolon at the end)
}

fn borrowing_slice(arr: [u8; 6], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length - {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
