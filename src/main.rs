#![allow(unused_variables)]
#![allow(dead_code)]

use std::io;

#[derive(Debug)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
struct Task {
    description: String,
    status: Status,
}

fn read_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>) {
    let desc = read_input("Enter Task Description : ");

    let task = Task {
        description: desc,
        status: Status::Pending,
    };

    tasks.push(task);
    println!("task added")
}

fn main() {
    println!("hello world");
}
