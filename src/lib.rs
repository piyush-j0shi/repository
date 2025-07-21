// one thing to know whenevr we implement the trait Iterator for custom or our own functions we
// need to do one thing which is define `type Item` which is for what will the Iterator yield or
// return whenever it is being consumed.

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// we need to make v1_iter mutale because this code consumes, or uses up, the iterator.
// Each call to next eats up an item from the iterator
// We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership
// of v1_iter and made it mutable behind the scenes.

// so to simply put it next method is like it takes a chip and eats it out of a box of chips now
// that chip can not go back so with the same analogy we need to think like we can just remove that
// element after yielding and its not possible without mutablilty.

#[test]
fn iter_demonstrates() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
