fn main() {
    // Tuples -> can have different data types, where is the tuple allocated in the memory
    let tup: (i32, f32, bool) = (500, 12.12, true);

    let (_x, y, _z) = tup;

    let integer: i32 = tup.0;
    let _floatpoint: f32 = tup.1;
    let _booleanvalue: bool = tup.2;

    println!("The value of integer is {integer}");
    println!("The value of y is {y}");

    // Array -> must have same data types, they are allocated inside the stack rather than the heap which means they are fixed in size
    // once they are initialized
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{a:?}");
    println!("{b:?}");

    let c = [3; 5];
    println!("{c:?}"); // this will print array c with 5 3's inside of them

    let result = c[0];
    println!("{result}");
    // rust compiler will first check if the index is within the length of the array before accessing the array itself, this is the memory safety principle applied by the rust compiler
}
