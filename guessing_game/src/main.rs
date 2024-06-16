use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // infinite loop
    loop {
        println!("Enter your guess number. ");
        // variables are immutable and using mut keyword in front makes it mutable
        // guess points to an empty instance of the String
        let mut guess = String::new();

        // the stdin returns a Standard input handler which can read from standard input
        io::stdin()
            // the read_line takes whatever the user inputs and then appends that to the string that we pass as the argument, not overwrite
            // references are also immutable by default so we use the &mut keyword
            // the read_line returns a Result value which is of type enumeration
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we are shadowing the variable guess
        // the parse() function returns a Result similar to the read_line function which means it has two variants Ok, err
        // the match will match the result of the statement guess.trim().parse() with the values inside
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a valid number");
                continue;
            }
        };

        println!("you have guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Congrats you have guessed the number");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
