use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// propagating errors
// fn read_username_from_file() -> Result<String, io::Error> {
//    let username_file_result = File::open("hello.txt");
//
//    let mut username_file = match username_file_result {
//        Ok(file) => file,
//        Err(error) => return Err(error),
//    };

//    let mut username = String::new();

//    match username_file.read_to_string(&mut username) {
//        Ok(_) => Ok(username),
//        Err(e) => Err(e),
//    }
//}

// Shorter version of the above function
// fn read_username_from_file() -> Result<String, io::Error> {
//    let mut username_file = File::open("hello.txt")?;
//    let mut username = String::new();
//    username_file.read_to_string(&mut username)?;
//    Ok(username)
//}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    // recoverable errors with `enum Result`

    // enum Result<T, E> {
    // Ok(T),
    // Err(E),
    // }

    // In the above enum `T` represents the type of value returned in case of success,
    // and `E` represents the type of error in case of failure.

    // if this function succeeds the value `greeting_file_name` will be an instance of `result OK`,
    // otherwise it will be an instance of `result ERR`

    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    // Ok(file) => file,
    // Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // another way to write same logic

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    // if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //       })
    //    } else {
    //        panic!("Problem opening the file: {error:?}");
    //   }
    // });

    // other way instead of result -> `unwrap or expect`

    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt")
    //    .expect("hello.txt should be included in this project");

    // Propagating Errors
    let result = read_username_from_file();

    match result {
        Ok(username) => println!("Username: {}", username),
        Err(e) => eprintln!("Failed to read username: {}", e),
    }
}
