#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
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

fn view_task(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("no task added");
        return;
    }

    for (index, value) in tasks.iter().enumerate() {
        let status = match value.status {
            Status::Done => "Done",
            Status::Pending => "Pending",
        };
        println!(
            "task number : {}, task description: {:?}, task status : {:?} ",
            index + 1,
            value.description,
            value.status
        );
    }
}

fn mark_done(tasks: &mut Vec<Task>) {
    view_task(tasks);
    let input = read_input("enter task number to mark as done: ");
    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
            return;
        }
        tasks[index - 1].status = Status::Done;
        println!("task marked as done");
    } else {
        println!("invalid input");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    view_task(tasks);
    let input = read_input("enter task number to delete : ");
    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("invalid task number");
            return;
        }
        tasks.remove(index - 1);
        println!("test deleted");
    } else {
        println!("invalid input");
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n===== to-do list =====");
        println!("add task");
        println!("view tasks");
        println!("mark as done");
        println!("delete task");
        println!("exit");

        let choice = read_input("enter your choice: ");

        match choice.trim() {
            "add task" => add_task(&mut tasks),
            "view tasks" => view_task(&tasks),
            "mark as done" => mark_done(&mut tasks),
            "delete task" => delete_task(&mut tasks),
            "exit" => {
                println!("goodbye");
                break;
            }
            _ => println!("invalid choice"),
        }
    }
}
