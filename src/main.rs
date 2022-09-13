struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // rust knows that the type of self is rectangle due to this methods being inside the impl Rectangle Context. We use & because we don't want to take the ownership we just want to read the data.
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the are of the rectangle is {} square pixels", rect1.area());
}
