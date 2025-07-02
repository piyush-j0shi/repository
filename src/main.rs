// // 2. struct + enum
// #[derive(Debug)]
// enum TaskStatus {
//     Todo,
//     InProgress,
//     Done,
// }

// #[derive(Debug)]
// struct Task {
//     name: String,
//     status: TaskStatus,
// }

// impl Task {
//     fn new(name: &str) -> Self {
//         Task {
//             name: name.to_string(),
//             status: TaskStatus::Todo,
//         }
//     }

//     fn complete(&mut self) {
//         self.status = TaskStatus::Done;
//     }
// }

// fn main() {
//     let mut task_1 = Task::new("username");
//     println!("{:?}", task_1);

//     let _status = task_1.complete();
//     println!("{:?}", task_1);
// }

fn main() {
    println!("struct + enum");
}
