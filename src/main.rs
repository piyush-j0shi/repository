trait Descriable {
    fn new() -> Self;
    fn add(&mut self, val: String);
    fn describe(&self);
}

struct Person {
    name: Vec<String>,
}

impl Descriable for Person {
    fn new() -> Self {
        Person { name: Vec::new() }
    }

    fn add(&mut self, value: String) {
        self.name.push(value);
    }

    fn describe(&self) {
        println!("person named : {:?}", self.name)
    }
}

fn main() {
    let mut person = Person::new();
    person.add(String::from("hello"));
    person.describe();
}
