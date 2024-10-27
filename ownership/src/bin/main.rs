fn main() {
    let s = String::from("Hello, World");

    take_ownership(s);

    let i = 10;

    makes_copy(i);
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
