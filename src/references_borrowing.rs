fn main() {
    // immutable reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("S is {s}");

    multiple_borrows();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// can't have multiple borrows with immutable references
fn multiple_borrows() {
    let mut s = String::from("borrowed twice");

    let r1 = &mut s;

    // cant do this!
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    // can do this!
    let r2 = &mut s;

    println!("{}", r2);


    {
        let r3 = &mut s;
    } // r3 goes out of scope here, so we can make a new reference with no problems.

    let r4 = &mut s;
    println!("{}", r4);


    let immutable_r1 = &s; // no problem
    let immutable_r2 = &s; // no problem
    println!("{}, {}", immutable_r1, immutable_r2);
}