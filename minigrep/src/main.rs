use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let (query, file_path) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    // println!("searching for {query}");
    // println!("in file {file_path}");

    // let contents =
    // fs::read_to_string(file_path).expect("something have been able to read the file");

    // println!("with text:\n{contents}");

    (query, file_path)
}
