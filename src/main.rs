fn main() {
    // Rc<T> enables the multiple owner of same data.
    // Box<T> allows immutable and mutable borrow checked at compile time.

    // RefCell<T> allows mutable borrow checked at runtime, you can mutatate the value inside the
    // RefCall<T> even when the RefCell<T> is immutable.
}
