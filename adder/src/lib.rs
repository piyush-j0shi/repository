// using panic()! and assert_eq()!

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("make this test fail");
//     }
// }

// Using assert!()

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// #[allow(dead_code)]
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[allow(dead_code)]
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };

//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller))
//     }

//     #[test]
//     fn larger_cannot_hold_smaller() {
//         let larger = Rectangle {
//             width: 1,
//             height: 1,
//         };

//         let smaller = Rectangle {
//             width: 2,
//             height: 2,
//         };

//         assert!(smaller.can_hold(&larger))
//     }
// }

// Using assert_eq!() and assert_ne()!

// pub fn add_two(a: usize) -> usize {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//    use super::*;

//    #[test]
//    fn it_adds_two() {
//        let result = add_two(2);
//        assert_eq!(result, 4);
//    }

//    #[test]
//    fn it_doesnot_add_two() {
//        let result = add_two(2);
//        assert_ne!(result, 5);
//    }
// }

// adding custom failure messages
// pub fn greeting(name: &str) -> String {
//     format!("hello")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("carol");
//         assert!(
//             result.contains("carol"),
//             "greeting did not contained name, value was `{result}`"
//         );
//     }
// }

// should_panic
// if we use `expected` with should_panic then we need to make sure that the string used in
// expected should be substring of original panic messages or whatever othervise the test cases
// will fail with an error.

// pub struct Guess {
//     value: i32,
// }
//
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("value must be greater than or equal to 1, got {value}");
//         } else if value > 100 {
//             panic!("value must be less than or equal to 100, got {value}");
//         }
//
//         Guess { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// test cases in `Result<T, E>` format

// fn add_two(value: i32) -> i32 {
//     value + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() -> Result<(), String> {
//         let result = add_two(2);
//
//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("something went wrong."))
//         }
//     }
// }

// Running tests in parllelly or consecutively
// We set the number of test threads to 1, telling the program not to use any parallelism.
// Running the tests using one thread will take longer than running them in parallel,
//  but the tests won’t interfere with each other if they share state.

// cargo test -- --test-threads=1

// printing function outputs in test
// cargo test -- --show-output

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("i got the value {a}");
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(10);
//         assert_eq!(value, 10)
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(5);
//         assert_eq!(value, 5)
//     }
// }

// running a subset of test by name
// 1. running a single test
// cargo test test_function_name (ex : cargo test add_two_and_one_hundred)

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_tw0() {
        let value = add_two(2);
        assert_eq!(value, 4)
    }

    #[test]
    fn add_two_and_three() {
        let value = add_two(3);
        assert_eq!(value, 5)
    }

    #[test]
    fn add_two_and_one_hundred() {
        let value = add_two(100);
        assert_eq!(value, 102)
    }
}
