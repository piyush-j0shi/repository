use ::std::fs::File;
use std::io::ErrorKind;

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
}
