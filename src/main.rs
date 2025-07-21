// iterators in rust are the patterns that allows you to perform some tasks on a sequence of items
// in turn.

// but in rust iterators are lazy, meaning they have no effect until you call the method that
// consumes the iterators to use it up.

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("got : {val}");
    }
}
