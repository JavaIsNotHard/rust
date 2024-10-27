fn main() {
    // slice lets you reference a sequence of elements in a collection, it is a kind of reference than a ownership

    let some_string: String = String::from("Hello World");

    let hello = &some_string[0..5];
    // or
    let hello2 = &some_string[..5];
    let world = &some_string[6..11];
    // or
    let world2 = &some_string[6..];

    println!("{hello} \n{world}");

    let second_string = first_word(&some_string);
    println!("{second_string}");
}

fn first_word(input: &String) -> &str {
    let bytes = input.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[0..index];
        }
    }

    &input[..]
}

fn second_word(input: &String) -> &str {
    let bytes = input.as_bytes();
    let mut count: usize = 0;
    let mut first_index: usize = 0;

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == 2 {
                return &input[first_index..index + 1];
            } else {
                count += 1;
                first_index = index;
            }
        }
    }

    &input[..]
}
