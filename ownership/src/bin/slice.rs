fn main() {
    // slice lets you reference a sequence of elements in a collection, it is a kind of reference than a ownership

    let some_string: String = String::from("Hello World");
    println!("{some_string}");

    let hello = &some_string[0..5];
    // or
    let _hello2 = &some_string[..5];
    let world = &some_string[6..11];
    println!("{world}");
    // or
    let _world2 = &some_string[6..];

    println!("{hello} \n {world}");

    // the second_string var holds a mutable reference to the string
    let second_string = first_word(&some_string);

    println!("{second_string}");
    // some_string.clear(); // this is not allowed becuase second_string is still used after this call
    // if there were no calls to second_string then we could clear the some_string
}

// if we use &str instead of &String then we can pass both String and str as input to the function this wouldn't have been possible using String as a parameter type
fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[0..index];
        }
    }

    &input[..]
}
