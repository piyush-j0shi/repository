#[allow(dead_code)]
#[derive(Debug)]
// <T> in rust
struct Point<T> {
    x: T,
    y: T,
}

// implementinf `impl` also
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
#[derive(Debug)]
// here we are using multiple generics so two generics <T> and <U> we can define as much as we want
// now the good thing is T and U can be same or different types
struct Point2<T, U> {
    x: T,
    y: U,
}

// as we have previously seen enums
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

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
// fn largest<T>(list : &[T]) -> &T{}

// we need to use `PartialOrd` because we want to compare the items in the list and rust does not
// know the types so othervise it will throw an error that I can not compare char with Int so to
// survive from that we use PartialOrd

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let ineteger = Point { x: 10, y: 10 };
    let float = Point { x: 10.0, y: 10.0 };
    println!("Integer Point: {:?}", ineteger);
    println!("Float Point: {:?}", float);

    // let wont_work = Point { x: 5, y: 4.0 };
    // This above line will throw an error bcz rust does not allow this both values should be same
    // type <T>

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    println!("Both Integer Point: {:?}", both_integer);
    println!("Both Float Point: {:?}", both_float);
    println!("Integer and Float Point: {:?}", integer_and_float);

    let p = Point { x: 5, y: 5 };
    println!("p.x = {}", p.x());
}
