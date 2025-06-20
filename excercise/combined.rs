use std::collections::HashMap;

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Banned,
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    status: Status,
}

fn main() {
    let mut user_map: HashMap<String, Vec<User>> = HashMap::new();

    user_map.insert(
        "admin".to_string(),
        vec![
            User {
                id: 1,
                name: "user1".to_string(),
                status: Status::Active,
            },
            User {
                id: 2,
                name: "user2".to_string(),
                status: Status::Inactive,
            },
        ],
    );

    user_map.insert(
        "guest".to_string(),
        vec![User {
            id: 3,
            name: "user3".to_string(),
            status: Status::Inactive,
        }],
    );

    for (group, users) in &user_map {
        println!("group : {:?}", group);
        for user in users {
            println!(
                "User ID: {}, Name: {}, Status: {:?}",
                user.id, user.name, user.status
            );
        }
    }

    // adding new user to admin

    let new_user = User {
        id: 4,
        name: "user4".to_string(),
        status: Status::Banned,
    };

    user_map
        .entry("admin".to_string())
        .or_insert_with(Vec::new)
        .push(new_user);

    println!("\nAfter adding new user to admin group : ");
    if let Some(admin) = user_map.get("admin") {
        for user in admin {
            println!(
                "User ID: {}, Name: {}, Status: {:?}",
                user.id, user.name, user.status
            );
        }
    } else {
        println!("No users found in admin group.");
    }

    // change status of a guest user
    if let Some(guest) = user_map.get_mut("guest") {
        for user in guest.iter_mut() {
            if user.name == "user3" {
                user.status = Status::Inactive;
            }
        }
    }

    println!("\nFinal State:");
    for (group, users) in &user_map {
        println!("Group: {}", group);
        for user in users {
            println!("  - {:?} (Status: {:?})", user.name, user.status);
        }
    }
}
