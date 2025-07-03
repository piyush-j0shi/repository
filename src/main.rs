#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress,
    Completed,
}

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(name: &str, description: &str) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            status: TaskStatus::Todo,
        }
    }

    fn start(&mut self) {
        self.status = TaskStatus::InProgress
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Completed
    }
}

fn main() {
    let mut task = Task::new("name", "description");
    println!(
        "task_name : {}, task_description : {}, status : {:?}",
        task.name, task.description, task.status
    );

    task.start();
    println!(
        "task_name : {}, task_description : {}, status : {:?}",
        task.name, task.description, task.status
    );

    task.complete();
    println!(
        "task_name : {}, task_description : {}, status : {:?}",
        task.name, task.description, task.status
    );
}
