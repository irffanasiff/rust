mod random_info;
use random_info::*;

#[allow(unused_variables)]
fn main1() {
    // strings
    { /*
             both String and&string are a grouping of characters
             differences
                 how they are stored in memory'
                 how programmer uses
             Strings
                 Heap
                 Mutable
             &str is more complex
                 Immutable
                 often allocated on the stack sometimes a heap reference sometimes embedded in the code
             can translate between string and str
         */
    }
    let example_str: char = 'h';
    let mut mut_string = String::new();
    mut_string.push(example_str);
    mut_string.push_str(" hello world");

    let a: String = String::from("a");
    let b: String = String::from("b");
    let combine = a + &b;
    // println!("mut string {} combine {}", mut_string, combine);

    let example_string = String::from("hello world");

    let char_by_index = &example_string.chars().nth(2);
    match char_by_index {
        Some(c) => println!("found a char {} ", c),
        None => {}
    }
    let returned_data = some_function(2.2, 50);
    //  println!("returned data is {} ", returned_data);

    let mut hello = HelloWorld {
        some_bool: true,
        some_float: 10.3,
        some_int: 7,
    };

    hello.some_bool = false;

    let random_info_var = RandomInfo {
        some_bool: true,
        some_int: 3,
    };
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("this is a function {}, {}", param_a, param_b);
    10.1
}

fn _some_procedure(_param_e: f32, _param_b: i8) {
    println!("I'm in some procedure");
}

/*
 structs represents complex data types that you define
structs are like "object", but have differences
    rust doesnt have inheritance
    rust does have methods
    rust has 'Traits' - similar to polymorphism for object oriented
derive traits can be done using macros
*/

struct HelloWorld {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
}
