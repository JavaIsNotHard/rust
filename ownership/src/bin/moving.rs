fn main() {
    let s1 = gives_ownership(); // s1 = a reference to String created by gives_ownership

    let s2 = String::from("New Hello World"); // s2 = a refernce created inside the main function

    let s3 = takes_and_gives(s2); // s3 references string created by s2

    println!("{s3}")
} // s1 and s3 are dropped here because they have ownership inside this function call but s2 is moved to takes_and_gives here is not dropped

fn gives_ownership() -> String {
    let some_string = String::from("Hello World");
    return some_string;
} // some_string is dropped here

fn takes_and_gives(some_string: String) -> String {
    return some_string;
} // some_string is dropped here
