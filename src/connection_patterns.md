# Rust Concepts Connection Cheat Sheet

## Connection Patterns - How Things Link Together

### Pattern 1: Vec + Enum (Collection of States)
```rust
enum Status { Active, Inactive, Pending }

let statuses: Vec<Status> = vec![
    Status::Active,
    Status::Pending,
    Status::Inactive,
];

// Process each status
for status in statuses {
    match status {
        Status::Active => println!("Working"),
        Status::Pending => println!("Waiting"),
        Status::Inactive => println!("Stopped"),
    }
}
```

### Pattern 2: Struct + Enum (Object with State)
```rust
enum TaskStatus { Todo, InProgress, Done }

struct Task {
    name: String,
    status: TaskStatus,  // Struct CONTAINS enum
}

impl Task {
    fn new(name: &str) -> Self {
        Task {
            name: name.to_string(),
            status: TaskStatus::Todo,
        }
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Done;  // Change the enum inside struct
    }
}
```

### Pattern 3: Struct + Vec (Object with Collection)
```rust
struct TodoList {
    items: Vec<String>,  // Struct CONTAINS vec
}

impl TodoList {
    fn new() -> Self {
        TodoList { items: Vec::new() }
    }

    fn add(&mut self, item: String) {
        self.items.push(item);  // Modify vec inside struct
    }

    fn get_all(&self) -> &Vec<String> {
        &self.items  // Return reference to vec
    }
}
```

### Pattern 4: Generic Struct (Works with Any Type)
```rust
struct Container<T> {  // T means "any type"
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

// Usage - same struct, different types
let int_container = Container::new(42);
let string_container = Container::new("hello".to_string());
```

### Pattern 5: Generic Enum (Enum with Any Type)
```rust
enum Result<T, E> {  // T = success type, E = error type
    Ok(T),
    Err(E),
}

// Usage
let success: Result<i32, String> = Result::Ok(42);
let failure: Result<i32, String> = Result::Err("Something went wrong".to_string());

match success {
    Result::Ok(value) => println!("Got: {}", value),
    Result::Err(error) => println!("Error: {}", error),
}
```

### Pattern 6: Trait (Define Behavior)
```rust
trait Describable {
    fn describe(&self) -> String;
}

struct Person { name: String }
struct Car { model: String }

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person named {}", self.name)
    }
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("Car model {}", self.model)
    }
}

// Usage - same trait, different types
let person = Person { name: "Alice".to_string() };
let car = Car { model: "Toyota".to_string() };

println!("{}", person.describe());
println!("{}", car.describe());
```

### Pattern 7: Generic + Trait (Type Must Have Behavior)
```rust
trait Printable {
    fn print(&self);
}

struct Printer<T> {
    item: T,
}

impl<T> Printer<T>
where
    T: Printable,  // T must implement Printable
{
    fn new(item: T) -> Self {
        Printer { item }
    }

    fn print_item(&self) {
        self.item.print();  // We know T has print() method
    }
}
```

### Pattern 8: Vec + Trait (Collection of Things with Same Behavior)
```rust
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String { "Woof!".to_string() }
}

impl Animal for Cat {
    fn make_sound(&self) -> String { "Meow!".to_string() }
}

// Collection of anything that implements Animal
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    println!("{}", animal.make_sound());
}
```

### Pattern 9: Lifetimes (References Must Live Long Enough)
```rust
struct Book<'a> {  // 'a means "lifetime parameter"
    title: &'a str,   // This reference must live as long as 'a
}

impl<'a> Book<'a> {
    fn new(title: &'a str) -> Self {
        Book { title }
    }

    fn get_title(&self) -> &str {
        self.title
    }
}

// Usage
let title = "Rust Programming".to_string();
let book = Book::new(&title);  // book can't outlive title
println!("{}", book.get_title());
```

### Pattern 10: All Together (The Full Combo)
```rust
trait Processable {
    fn process(&self) -> String;
}

enum Status { Success, Error }

struct Processor<'a, T> {
    name: &'a str,
    items: Vec<T>,
}

impl<'a, T> Processor<'a, T>
where
    T: Processable,
{
    fn new(name: &'a str) -> Self {
        Processor {
            name,
            items: Vec::new()
        }
    }

    fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn process_all(&self) -> Vec<Status> {
        self.items.iter()
            .map(|item| {
                let result = item.process();
                if result.is_empty() {
                    Status::Error
                } else {
                    Status::Success
                }
            })
            .collect()
    }
}
```

## Quick Connection Rules

1. **Struct contains other things** → Use struct fields
2. **Multiple things of same type** → Use Vec<T>
3. **Different possible states** → Use enum
4. **Work with any type** → Use generics <T>
5. **Ensure type has behavior** → Use trait bounds
6. **Hold references** → Use lifetimes <'a>

## Common Connection Patterns

| Pattern | Code | Purpose |
|---------|------|---------|
| **Struct + Enum** | `struct Thing { status: Status }` | Object with state |
| **Struct + Vec** | `struct List { items: Vec<T> }` | Object with collection |
| **Vec + Enum** | `Vec<Status>` | Collection of states |
| **Generic Struct** | `struct Container<T>` | Reusable container |
| **Trait Bound** | `fn work<T: Trait>(item: T)` | Ensure behavior |
| **Lifetime** | `struct Ref<'a> { data: &'a str }` | Safe references |

## Mental Model

Think of it like LEGO blocks:
- **Struct** = Custom LEGO piece that holds other pieces
- **Enum** = Piece that can be one of several shapes
- **Vec** = Bag that holds many pieces of the same type
- **Generic** = Piece that works with any color
- **Trait** = Guarantee that piece has certain features
- **Lifetime** = Rules about how long pieces stay connected

The key is that these pieces **snap together** in predictable ways!
