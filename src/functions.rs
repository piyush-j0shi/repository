use std::io;

fn main() {
    println!("another function is ");
    another_function();
    argpr_function(2);
    labeled_measurements(2, 'h');

    println!("enter a number");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read value");
    let input: i32 = input.trim().parse().expect("no a number");

    let plus = plus_one(input);
    println!("the plus_one is : {plus}");
}

fn another_function() {
    println!("this is an another_function");
}

fn argpr_function(x: i32) {
    println!("the value of x is : {x}");
}

fn labeled_measurements(value: i32, unit_label: char) {
    println!("the labeled_measurements is : {value}{unit_label}",);
}

fn plus_one(y: i32) -> i32 {
    y + 1
}
