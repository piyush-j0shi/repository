fn main() {
    // panic! macro
    // panic!("crash and burn");

    // panic without using `panic!` macro
    let v = vec![1, 2, 3];
    v[99];

    // use this following command to check the full output
    // RUST_BACKTRACE=1 cargo run
    // here the lines above that spot are code that your code has called,
    // and then lines below are the code that called your code.
}
