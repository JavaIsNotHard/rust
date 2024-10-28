fn main() {
    let mut v: Vec<u32> = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third: &u32 = &v[2];
    println!("{third}");

    let second: Option<&u32> = v2.get(1);
    match second {
        Some(second) => println!("{second:?}"),
        None => println!("couldn't find the value at the index"),
    };

    let mut newVec = vec![1, 2, 3, 4, 5];
    let first = &newVec[3];
    // newVec.push(20); // this is not possible
    println!("The value of the fourth element is {first}"); // the first variable is an immutable reference and this is the last usage of this usage
    newVec.push(10); // this is possible

    for items in &mut newVec {
        *items += 1;
        println!("{items}");
    }
}
