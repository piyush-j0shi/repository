mod task;
mod task_ops;
mod utils;

use task::Task;
use task_ops::*;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n===== To-Do List =====");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark as Done");
        println!("4. Delete Task");
        println!("5. Change Priority");
        println!("6. Exit");

        let choice = utils::read_input("Enter your choice: ");
        match choice.as_str() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_done(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => change_priority(&mut tasks),
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}
