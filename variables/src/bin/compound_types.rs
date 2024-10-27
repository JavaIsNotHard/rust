fn main() {
    // Tuples -> can have different data types
    let tup: (i32, f32, bool) = (500, 12.12, true);

    let (_x, y, _z) = tup;

    let integer: i32 = tup.0;
    let _floatpoint: f32 = tup.1;
    let _booleanvalue: bool = tup.2;

    println!("The value of integer is {integer}");
    println!("The value of y is {y}");

    // Array -> must have same data types
}
