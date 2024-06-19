fn main() {
    // ownership in rust
    // Rules of ownership in rust
    // 1. Every value in rust has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner gets out of scope, the value is also dropped

    {
        // s is not valid here
        let s = "Hello"; // s is valid here
    } // the scope is over and s is no longer valid

    // every variable are immutable by default
    // this is string literal
    let mut new_string = "Hello, World!";
    // shadow copy not changing the original new_string variable
    new_string = "new world";

    // this is String type
    let mut s = String::from("Hello, ");
    s.push_str("World!");

    // not shallow copy but rather move
    // this means that once we assign a new variable to point to the string content on the heap then the variable previously pointing to that string content will be no longer valid
    let string1 = String::from("Hello");
    let string2 = string1;
    // not allowed
    // println!(string1);

    // creates a deep copy
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    // here x and y are of integer type and are allocated on the stack
    // a type that is stored on the stack has the Copy trait
    // if a variable has the Copy trait then it does not move but rather performs trivial copies
    let x = 6;
    let y = x;

    // newstring is sotred in heap and so has the Drop trait which means we cannot annotate this type with the Copy trait
    let newstring = String::from("newstring");

    // onwership and functions
    let s = String::from("helo");
    takes_ownership(s); // s'v value is valued into the takes_ownership function
                        // s is invalid from here

    let x = 5;
    makes_copy(x); // x is moved in the makes_copy fuction but i32 is a copy so its okay to still use x afterwards
                   // x is still valid here

    let s1 = gives_ownership(); // the gives_ownership fucntion gives the ownership of the String to the main function
    let s2 = String::from("hello"); //
    let s3 = takes_and_gives_back(s2); // s2 is moved so nothing special happens when the scope runs out, s1 is dropped and s3 is dropped as well

    // How a function take a value but not take the ownership of the value
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

// this moves the ownership of the String to the calling function
fn gives_ownership() -> String {
    let some_string = String::from("Hello"); // some_string comes into scope
    some_string
}

// this function takes the ownership of a_string and then immediately gives the ownership back to the calling function
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope and hence is dropped

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer goes out of scope but nothing happens
