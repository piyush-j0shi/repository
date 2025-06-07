// #[derive(Debug)]

// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cent(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //    let coin = Coin::Quarter;
    //        let value = value_in_cent(coin);
    //        println!("The value of the coin is: {} cents", value);

    // let coin = Coin::Quarter(UsState::Alaska);
    // println!("The value is {} cents", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {:?}, six: {:?}, none: {:?}", five, six, none);

    let dice_roll = 9;

    //    Each => arm in the match expression expects an expression
    match dice_roll {
        3 => println!("it adds fancy hat"),
        7 => println!("it removes fancy hat"),
        _ => println!("it rerolls"),
    }
}
