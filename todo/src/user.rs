use crate::task::Task;
use crate::utils::read_input;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub tasks: Vec<Task>,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            tasks: Vec::new(),
        }
    }
}

pub fn login(users: &mut Vec<User>) -> Option<&mut User> {
    let username = read_input("Username: ");
    let password = read_input("Password: ");

    for user in users.iter_mut() {
        if user.username == username && user.password == password {
            println!("Login successful. Welcome, {}!", username);
            return Some(user);
        }
    }

    println!("Invalid credentials.");
    None
}
