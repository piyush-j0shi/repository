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

// #[derive(Debug)]
// struct ItemsList<T> {
//     items : T,
// }

#[derive(Debug)]
struct ItemList<T> {
    items: Vec<T>,
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

trait Describable {
    fn describe(&self) -> String;
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

    // fn todo_count(&self) -> usize {
    //     self.tasks.iter()
    //         .filter(|task| matches!(task.status, TaskStatus::Todo))
    //         .count()
    // }

    // fn inprogress_count(&self) -> usize {
    //     self.tasks.iter()
    //         .filter(|task| matches!(task.status, TaskStatus::InProgress))
    //         .count()
    // }

    // fn completed_count(&self) -> usize {
    //     self.tasks.iter()
    //         .filter(|task| matches!(task.status, TaskStatus::Completed))
    //         .count()
    // }
}

// impl <T> ItemsList<T> {
//     fn new(item : T) -> Self {
//         ItemsList { items: item }
//     }
// }

// impl<T> ItemList<T> {
//     fn new() -> Self {
//         ItemList { items: Vec::new() }
//     }
//
//     fn add(&mut self, item: T) {
//         self.items.push(item);
//     }
//
//     fn count(&self) -> usize {
//         self.items.len()
//     }
//
//     fn get_all(&self) -> &Vec<T> {
//         &self.items
//     }
// }

impl Describable for Task {
    fn describe(&self) -> String {
        format!("{}, {}", self.name, self.description)
    }
}

impl Describable for User {
    fn describe(&self) -> String {
        format!("{}, {}", self.name, self.email)
    }
}

impl<T> ItemList<T>
where
    T: Describable,
{
    fn new() -> Self {
        ItemList { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn describe_all(&self) {
        for item in &self.items {
            println!("{}", item.describe());
        }
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

    // let todotask = tasklist.todo_count();
    // let inprogresstask = tasklist.inprogress_count();
    // let completedtask = tasklist.completed_count();

    // println!("todo task count : {}", todotask);
    // println!("inprogress task count : {}", inprogresstask);
    // println!("completed task count : {}", completedtask);

    // let itemlist = ItemsList::new(3);
    // println!("{:?}", itemlist.items);

    //     let mut itemlist = ItemList::new();
    //     itemlist.add("item");
    //     itemlist.add("item");
    //     itemlist.add("item");
    //     itemlist.add("item");
    //
    //     let itemcount = itemlist.count();
    //     let getallitem = itemlist.get_all();

    //     println!("items : {:#?}", itemlist);
    //     println!("items : {}", itemcount);
    //     println!("items : {:#?}", getallitem);

    let mut itemlist = ItemList::<Task>::new();
    let mut userlist = ItemList::<User>::new();

    itemlist.add(Task::new("name", "description"));
    userlist.add(User {
        name: "name".to_string(),
        email: "email".to_string(),
    });

    itemlist.describe_all();
    userlist.describe_all();
}
