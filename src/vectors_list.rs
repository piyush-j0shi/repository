// allow clippy
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
// these are to suppress warnings
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // it's structure is like `Vec<T>`  and here T stands for type.
    let v: Vec<i32> = Vec::new();

    // Vec! -> Macro
    let v1 = vec![1, 2, 3];

    // pushing into a vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("v : {:?}", v);
    println!("v1 : {:?}", v1);
    println!("v2 : {:?}", v2);

    // reading element of vectors
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[3];
    println!("the third element is {third}");

    let fourth: Option<&i32> = v4.get(2);
    match fourth {
        Some(fourth) => println!("this is the fourth element : {fourth}"),
        None => println!("there is no fourth element"),
    }

    // programme will panic
    // let v5 = vec![1, 2, 3, 4];
    // let does_not_exists = &v[100];
    // let does_not_exists_1 = v.get(100);

    // you can not do this because this goddam borrow checkers, But tbh borrow checkers are goated.

    // let mut v6 = vec![1, 2, 3, 4, 5];
    // let first = &v6[0];
    // v6.push(6);
    // println!("the first element is : {first}");

    //iterating the values

    let v7 = vec![100, 32, 57];
    for i in &v7 {
        println!("i : {i}");
    }

    //iterations over mutable references

    let mut v8 = vec![1, 2, 3];
    for i in &mut v8 {
        println!("old i : {i}");
        *i += 50;
        println!("new i : {i}")
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
