#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]

use std::io;

#[derive(Debug)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
enum Priority {
    Low,
    High,
    Medium,
}

#[derive(Debug)]
struct Task {
    description: String,
    status: Status,
    priority: Priority,
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
    let desc = read_input("enter task description : ");

    println!("available priorities");
    println!("Low");
    println!("Medium");
    println!("High");

    let prrt = read_input("enter task priority : ");

    let task = Task {
        description: desc,
        status: Status::Pending,
        priority: match prrt.as_str() {
            "Low" => Priority::High,
            "Medium" => Priority::Medium,
            "High" => Priority::High,
            _ => {
                println!("invalid output setting default priority");
                Priority::Low
            }
        },
    };

    tasks.push(task);
    println!("task added");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("no task found");
        return;
    }

    for (index, value) in tasks.iter().enumerate() {
        let _status = match value.status {
            Status::Done => "Done",
            Status::Pending => "Pending",
        };

        let _priority = match value.priority {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };

        println!(
            "task number : {:?}, description : {:?}, status : {:?}, priority : {:?}",
            index + 1,
            value.description,
            value.status,
            value.priority,
        );
    }
}

fn mark_done(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
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

fn change_priority(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
    let input = read_input("enter task number to change priority : ");
    let change_priority = read_input("enter the priority to want to change to : ");

    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
            return;
        }

        match change_priority.as_str() {
            "Low" => tasks[index - 1].priority = Priority::Low,
            "Medium" => tasks[index - 1].priority = Priority::Medium,
            "High" => tasks[index - 1].priority = Priority::High,
            _ => {
                println!("no options available like this");
                tasks[index - 1].priority = Priority::Low
            }
        }

        println!("priority changed");
    } else {
        println!("invalid input");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
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
        println!("1. add task");
        println!("2. view tasks");
        println!("3. mark as done");
        println!("4. delete task");
        println!("5. change priority");
        println!("6. exit");

        let choice = read_input("enter you choice input number : ");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_done(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => change_priority(&mut tasks),
            "6" => {
                println!("goodbye");
                break;
            }
            _ => println!("invalid choice"),
        }
    }
}
