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

#[derive(Debug)]
struct TaskList {
    tasks: Vec<Task>,
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

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
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

    let mut tasklist = TaskList::new();
    tasklist.add_task(task);
    println!("{:#?}", tasklist);
}
