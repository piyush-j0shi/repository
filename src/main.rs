use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

// unwrap_or_else takes a little function that gives a fallback value.
// If there's already a value (Some), it just uses that.
// If there's no value (None), it runs your fallback function to get one.

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user1_preference = Some(ShirtColor::Red);
    let user2_preference = None;

    let giveaway1 = inventory.giveaway(user1_preference);
    let giveaway2 = inventory.giveaway(user2_preference);

    println!(
        "User 1 gets: {:?} with preference : {:?}",
        giveaway1, user1_preference
    );
    println!(
        "User 2 gets: {:?} with preference : {:?}",
        giveaway2, user2_preference
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // we can not use this below code because the closure we are using will not accept the other
    // argument than string because it locks in the type of first argument.

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    println!("Result: {}", expensive_closure(42));

    // closure can take values from environment in three ways which are
    // 1. borrowing immutably
    // 2. borrowing mutably
    // 3. taking ownership

    // 1. borrowing immutably

    let list = vec![1, 2, 3];
    println!("before defining closure : {list:?}");
    let only_borrow = || println!("from closure : {list:?}");
    println!("before calling closure : {list:?}");
    only_borrow();
    println!("after callling closure : {list:?}");

    // 2. borrowing mutably

    let mut list1 = vec![1, 2, 3];
    println!("before defining closure : {list:?}");
    let mut borrow_mutably = || list1.push(7);
    println!("before calling closure : {list:?}");
    borrow_mutably();
    println!("after callling closure : {list:?}");
}
