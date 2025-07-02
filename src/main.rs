trait Descriable {
    fn new() -> Self;
    fn add(&mut self, val: String, status: String);
    fn edit(&mut self, newstatus: String);
    fn describe(&self);
}

#[derive(Debug)]
enum Progress {
    Pending,
    Completed,
}

#[derive(Debug)]
struct Person {
    name: Vec<String>,
    progress: Progress,
}

impl Descriable for Person {
    fn new() -> Self {
        Person {
            name: Vec::new(),
            progress: Progress::Pending,
        }
    }

    fn add(&mut self, value: String, status: String) {
        self.name.push(value);
        match status.as_str() {
            "pending" => self.progress = Progress::Pending,
            "completed" => self.progress = Progress::Completed,
            _ => println!("invalid choice"),
        }
    }

    fn edit(&mut self, newstatus: String) {
        match newstatus.as_str() {
            "pending" => self.progress = Progress::Pending,
            "completed" => self.progress = Progress::Completed,
            _ => println!("invalid choice"),
        }
    }

    fn describe(&self) {
        println!("person named : {:?}", self.name);
        println!("Progress : {:?}", self.progress);
    }
}

fn main() {
    let mut person = Person::new();
    person.add(String::from("values"), String::from("pending"));
    person.describe();

    person.edit(String::from("completed"));
    person.describe();
}
