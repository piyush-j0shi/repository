// 1. vec + enum
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {
    // 1. vec +
    let statues: Vec<Status> = vec![Status::Active, Status::Inactive, Status::Pending];

    for status in statues {
        match status {
            Status::Active => println!("active"),
            Status::Inactive => println!("inactive"),
            Status::Pending => println!("pending"),
        }
    }
}
