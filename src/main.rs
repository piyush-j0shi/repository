fn main() {
    // it's structure is like `Vec<T>`  and here T stands for type.
    let v: Vec<i32> = Vec::new();

    // Vec! -> Macro
    let v1 = vec![1, 2, 3];

    println!("v: {:?}", v);
    println!("v1: {:?}", v1);
}
