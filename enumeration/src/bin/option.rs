fn main() {
    let some_number = Some(5); // there is value
    let some_char = Some('c');

    // Option<T> is only used in the case where there may or may not be a value
    let absent_value: Option<i32> = None; // there is no value

    // let x: i8 = 5;
    // let null: Option<i8> = x; // this is not possible because i8 and Option<i8> are different types
    // having a i8 type guarantees that there is value for the variable
}
