fn main() {
    let _string_literal = "Hello World";

    let mut dynamic_string = String::from("Hello World");

    {
        let mut _new_string = String::from("Hello String"); // valid here
                                                            // do some work
    } // not valid from here, the compiler calls drop function automatically for us which will free up the memory for us

    dynamic_string.push_str("Jibesh");

    // this is a simple where booth x and y will have different values of 5
    let x = 5;
    let _y = x;

    let s1 = String::from("new string");
    let s2 = s1; // we say that s1 is moved into s2

    let s3 = s2.clone(); // creates a deep copy of the string which means this assignment does not invalidate s2 like s1

    // println!("{s1}"); // cannot do this because after s2 = s1, s1 is not longer considered valid
    println!("{s2}"); // but this is valid

    // if we had both s1 and s2 pointing to the same heap location then during drop function call, they will try to free the same memory region twice
}
