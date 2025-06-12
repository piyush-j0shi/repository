#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]

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
}
