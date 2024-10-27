const NEW_VARIABLE: u32 = 12; // the scope of constant is global  and will be valid till the end of the program

// variable declaration
pub fn main() {
    shadowing();
    constant();
}

fn shadowing() {
    let x = 5;
    let x = x + 1; // the new x shadows the pervious x variable 

    {
        let x = x * 2; // again shadow the previous x i.e 6 to x * 2 = 12
        println!("The new value is {x}");
    }

    println!("The new value is {x}");

    // this block is allowed
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    // this block is not allowed
    // let mut spacestwo = "   ";
    // spacestwo = spacestwo.len(); println!("{spacestwo}");
}

fn constant() {
    println!("{NEW_VARIABLE}");
}

fn data_types() {
    let x: u32 = 32;
    let x: usize = 120; // shadowing of the variable x
    let y: usize = 1_000_000; // similar to java

    let float_number: f32 = 12.32;

    let boolea_var: bool = true;

    let character_var: char = 'e';

    let emoji_char: char = '🤣';

    println!("{x}, {y}, {float_number}, {boolea_var}, {character_var}, {emoji_char}");
}
