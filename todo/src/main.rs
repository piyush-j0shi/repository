// src/main.rs

mod task;
mod task_ops;
mod user;
mod utils;

use task_ops::*;
use user::{User, login};

fn main() {
    let mut users = vec![User::new("user1", "1234"), User::new("user2", "abcd")];

    loop {
        println!("Please log in to continue.\n");

        if let Some(current_user) = login(&mut users) {
            loop {
                println!("\n===== {}'s todo =====", current_user.username);
                println!("1. Add Task");
                println!("2. View Tasks");
                println!("3. Mark as Done");
                println!("4. Delete Task");
                println!("5. Change Priority");
                println!("6. Logout");

                let choice = utils::read_input("Enter your choice: ");
                match choice.as_str() {
                    "1" => add_task(&mut current_user.tasks),
                    "2" => view_tasks(&current_user.tasks),
                    "3" => mark_done(&mut current_user.tasks),
                    "4" => delete_task(&mut current_user.tasks),
                    "5" => change_priority(&mut current_user.tasks),
                    "6" => {
                        println!("Logging out");
                        break;
                    }
                    _ => println!("Invalid option."),
                }
            }
        }
    }
}
