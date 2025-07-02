// 3. struct + vec
#[derive(Debug)]
struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { items: Vec::new() }
    }

    fn add(&mut self, item: String) {
        self.items.push(item);
    }

    fn get_all(&self) -> &Vec<String> {
        &self.items
    }
}

fn main() {
    let mut items = TodoList::new();
    items.add(String::from("value"));
    items.add(String::from("value"));
    items.add(String::from("value"));

    let all_items = items.get_all();
    println!("all items : {:?}", all_items);
}
