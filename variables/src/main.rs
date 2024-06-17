const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;

    {
        // the scope of this shadow variable is within the curly braces
        // that means once all the code inside the scope completes then it will revert back to its previous value
        let x = x * 2;
        println!("the value of x in the inner scope is {x}");
    }

    println!("The value of x in the outer scope is {x}");

    let spaces = "    ";
    // this is ok
    let spaces = spaces.len();

    let mut tabs = "    ";
    // this is not ok
    // tabs = tabs.len();

    let guess = "12";
    let guess: u32 = guess.parse().expect("provide a number as argument");

    let tup: (i32, f64, u8) = (500, 30.1, 12);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let thirty_point_one = tup.1;

    println!("The value of x, y and z are {x}, {y} and {z}");
    println!("The value of five hundred is {five_hundred}");
}
