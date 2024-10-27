fn main() {
    let mut s1 = String::from("Hello World");

    let len = calculate_length(&s1);

    change_mut(&mut s1);

    let s2 = &mut s1;
    let s3 = &mut s1;

    {
        let r1 = &mut s1;
    } // r1 goes out of scope so we can take a new reference

    let r2 = &mut s1; // this is allowed

    let mut new_string = String::from("Hello World");

    let new_string_ref1 = &new_string;
    let new_string_ref2 = &new_string;
    let new_string_mut_ref = &mut new_string; // this is not allowed, it already has a immutable reference

    let ref1 = &new_string;
    let ref2 = &new_string;

    println!("{ref1} and {ref2}"); // last usage of ref1 and ref2 and not used after this point
    let ref3 = &mut new_string; //

    println!("the len of s1 is {len}");
}

fn calculate_length(s: &String) -> usize {
    // s is borrowing from s1
    return s.len();
}

fn change(s: &String) -> String {
    let s = s.push_str("jibesh");
    return s; // cannot modify a reference
}

fn change_mut(s: &mut String) {
    s.push_str(" jibesh"); // can modify because we pass a mutable reference
}
