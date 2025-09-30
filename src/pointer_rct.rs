enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    // Rust has the smart pointer Rc<T>, The refrence counted smart pointer, It is an
    // abbrevation for `refrence counting`.

    //The Rc<T> type keeps track number of refrences to a value to determine weather or not the
    //value is still in use.

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::new(Cons(10, Rc::clone(&a)));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "Counting after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
