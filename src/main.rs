// 9. lifetimes

struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn new(title: &'a str) -> Self {
        Book { title }
    }

    fn get_title(&self) -> &str {
        self.title
    }
}

fn main() {
    let title = "Rust Programming".to_string();
    let book = Book::new(&title);
    println!("{}", book.get_title());
}
