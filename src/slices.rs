
fn main() {
    make_slice();


    let mut s = String::from("hello world");
    let first = first_word(&s);

    // s.clear(); // error! - multiple  borrow because clear takes a mutable ref

    println!("First word is {}", first);


    // slice of array
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn make_slice() {
    let str = String::from("james lewis");
    let first_name = &str[..5];
    let last_name = &str[6..];
    let whole_name = &str[..];

    println!("Hello {} {}", first_name, last_name);

    println!("Hello {}", whole_name);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}