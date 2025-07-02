// 6. Trait
trait Descriable {
    fn new(val: String) -> Self;
    fn describe(&self) -> String;
}

struct Person {
    name: String,
}
struct Car {
    model: String,
}

impl Descriable for Person {
    fn new(val: String) -> Self {
        Person { name: val }
    }

    fn describe(&self) -> String {
        format!("person named : {}", self.name)
    }
}

impl Descriable for Car {
    fn new(val: String) -> Self {
        Car { model: val }
    }

    fn describe(&self) -> String {
        format!("car model : {}", self.model)
    }
}

fn main() {
    let person = Person::new(String::from("person1"));
    println!("{:?}", person.describe());

    let car = Car::new(String::from("porsche 911 gt3 rs"));
    println!("{:?}", car.describe());
}
