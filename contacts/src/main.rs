mod actions;
mod model;
mod utils;

use actions::*;
use model::Contact;

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n===== contact list =====");
        println!("1. add contact");
        println!("2. view contact");
        println!("3. search contact");
        println!("4. delete contact");
        println!("5. edit name");
        println!("6. edit phone");
        println!("7. exit");

        let choice = utils::read_input("enter your choice: ");

        match choice.trim() {
            "1" => add_contact(&mut contacts),
            "2" => view_contacts(&contacts),
            "3" => search_contact(&contacts),
            "4" => delete_contact(&mut contacts),
            "5" => edit_name(&mut contacts),
            "6" => edit_phone(&mut contacts),
            "7" => {
                println!("goodbye");
                break;
            }
            _ => println!("invalid choice"),
        }
    }
}
