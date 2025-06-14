use crate::task::{Priority, Status, Task};
use crate::utils::read_input;

pub fn add_task(tasks: &mut Vec<Task>) {
    let desc = read_input("Enter task description: ");
    println!("Available priorities: Low, Medium, High");

    let input = read_input("Enter task priority: ");
    let priority = match input.as_str() {
        "Low" => Priority::Low,
        "Medium" => Priority::Medium,
        "High" => Priority::High,
        _ => {
            println!("Invalid priority, defaulting to Low.");
            Priority::Low
        }
    };

    let task = Task {
        description: desc,
        status: Status::Pending,
        priority,
    };

    tasks.push(task);
    println!("Task added.");
}

pub fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        println!(
            "Task {} | Description: {} | Status: {:?} | Priority: {:?}",
            i + 1,
            task.description,
            task.status,
            task.priority
        );
    }
}

pub fn mark_done(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
    let input = read_input("Enter task number to mark as done: ");

    if let Ok(index) = input.parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
        } else {
            tasks[index - 1].status = Status::Done;
            println!("Task marked as done.");
        }
    } else {
        println!("Invalid input.");
    }
}

pub fn change_priority(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
    let input = read_input("Enter task number to change priority: ");
    let new_priority = read_input("Enter new priority: ");

    if let Ok(index) = input.parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
            return;
        }

        let priority = match new_priority.as_str() {
            "Low" => Priority::Low,
            "Medium" => Priority::Medium,
            "High" => Priority::High,
            _ => {
                println!("Invalid input. Defaulting to Low.");
                Priority::Low
            }
        };

        tasks[index - 1].priority = priority;
        println!("Priority updated.");
    } else {
        println!("Invalid input.");
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    view_tasks(tasks);
    let input = read_input("Enter task number to delete: ");
    if let Ok(index) = input.parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
            return;
        }
        tasks.remove(index - 1);
        println!("Task deleted.");
    } else {
        println!("Invalid input.");
    }
}
