// 10. fulll combi
// trait Processable {
//     fn process(&self) -> String;
// }

// enum Status { Success, Error }

// struct Processor<'a, T> {
//     name: &'a str,
//     items: Vec<T>,
// }

// impl<'a, T> Processor<'a, T>
// where
//     T: Processable,
// {
//     fn new(name: &'a str) -> Self {
//         Processor {
//             name,
//             items: Vec::new()
//         }
//     }

//     fn add_item(&mut self, item: T) {
//         self.items.push(item);
//     }

//     fn process_all(&self) -> Vec<Status> {
//         self.items.iter()
//             .map(|item| {
//                 let result = item.process();
//                 if result.is_empty() {
//                     Status::Error
//                 } else {
//                     Status::Success
//                 }
//             })
//             .collect()
//     }
// }

fn main() {
    println!("ok");
}
