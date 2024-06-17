fn main() {
    another_function();
}

fn another_function() {
    print_something(12, 'c');
}

fn print_something(x: i32, name: char) {
    println!("{x} {name}");
}

// statements vs expressions
// statement do not return a value
fn some_main() {
    // this is a statement because let y = 6 does not return a value
    let y = 6;
    // so this is not allowed since let y = 5 does not return a value and hence we cannot bind x to a value
    // let x = (let y = 5);

    // this is possible because anything inside a curly braces are expressions
    let z = {
        let x = 1;
        // we do not have a semi colon here because that would convert it into a statement hence we do not return any value
        x + 2
    };
}

// function with return values
fn five() -> i32 {
    5
}
