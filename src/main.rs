// 4. generic struct
#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let new_values = Container::new(-2);
    println!("{:?}", new_values.get());
}
