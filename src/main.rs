//fn largest_i32(list: &[i32]) -> &i32 {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
//
//fn largest_char(list: &[char]) -> &char {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

// when you write a function that works for many types (Or to remove deduplication and to use
// Generic) so rust needs to know what kind of type you mean and you declare that with `<T>` and it
// not necessarily to be T but its like a convention you can use whatever you want but you need to
// make sure one thing it will use only one Letter and its should be capital because rust uses
// camelcase type

// if you want to remove deduplication from largest we can do it like
// fn largest<T>(list : *[T]) -> &T{}

fn main() {
    //    let number_list = vec![34, 50, 25, 100, 65];
    //    let result = largest_i32(&number_list);
    //    println!("The largest number is {result}");
    //
    //    let char_list = vec!['y', 'm', 'a', 'q'];
    //    let result = largest_char(&char_list);
    //    println!("The largest char is {result}");
}
