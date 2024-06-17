fn main() {
    let number = 4;

    // if is an expression
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    let condition = true;
    // since if is an expression so can return values from them
    let newnumber = if condition { 5 } else { 6 };
    // we cannot do this because the return value must have the same type
    // let newnumber  = if condition { 5 } else { "six" };

    println!("The number is {newnumber}");

    let mut counter = 0;

    // loop can return value
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 10;
        }
    };

    println!("The result is {result}");

    // if we have multiple nested loops then we can break the outer loop using a label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // exit the inner loop
                break;
            }
            if count == 2 {
                // exit the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];

    for elements in a {
        println!("the value is {elements}");
    }
}
