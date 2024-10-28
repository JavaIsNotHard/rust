enum Coin {
    Penny,
    Nicket,
    Dime,
    Quarter,
}

fn value_of_coin(coin: Coin) -> u32 {
    // match should have arm for all possible value of the matching expression which in this case is Coin
    match coin {
        Coin::Penny => 1,
        Coin::Nicket => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _other => {
            println!("default");
            return 0;
        }
    }
}

fn add_one(input: Option<i32>) -> Option<i32> {
    match input {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let result = value_of_coin(Coin::Quarter);

    let five = Some(5);
    let six = add_one(five);
    println!("{six:?}");

    let none = add_one(None);
    println!("{none:?}");

    println!("{result}");
}
