
fn main() {

    let tup: (i32, i32) = (10, 10);
    let tup0 = tup.0;

    println!("tup is {tup0}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];


    let first = &arr[0];
    let arr_ref = *first;
    println!("{arr_ref}");

    let res = another_function("Hello world", '🤓');
    println!("Response is {res}");

    control_flow_stuff();
    loopy();
    while_loop();
    for_loop();
}

fn loopy() {
    let mut counter = 0;

    let final_val = loop {
        if counter >= 10 {
            break counter;
        }

        println!("iteration {counter}");
        counter += 1;
    };

    println!("Final value is {final_val}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn control_flow_stuff() {
    let condition = true;
    let if_res = if condition  {
        "yes"
    } else {
        "no"
    };

    println!("if_res: {if_res}")
}

fn another_function(a: &str, some_char: char) -> &str {
    println!("Argument a is: {a} and some_char is {some_char}");
    {
        "hello yourself"
    }
}