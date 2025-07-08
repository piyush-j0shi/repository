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

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 to 100");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
