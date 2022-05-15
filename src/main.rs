//To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    println!("please input your guess.");

    /*
     *  new is an associated function of the String
     *  type. It returns a new String with the given
     *   capacity.
     */
    let mut guess = String::new();
    // line has created a mutable variable that is currently bound to a new, empty instance of a String

    // we call the stdin method in the io module which will take users input and store it in the guess variable
    io::stdin() // this can also be written as std::io::Stdin
        .read_line(&mut guess) // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. This is known as borrowing.
        // references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
        .expect("failed to read line");

    println!("you guessed: {}", guess);
    println!("{}", random_number())
    // match guess.cmp(random_number()) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
}

fn random_number() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
    return secret_number;
    //We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
}
