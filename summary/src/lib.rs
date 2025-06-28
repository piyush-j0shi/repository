// So basically this whole story revolves around one thing which is Trait is going to be a list of methods
// and if your type has those methods
// then you can use that trait for that type, But that type needs ti implement those methods
// now where to check if the type has those methods or not if custom then you know othervise
// look for docs or rust compiler will let you know baby.
// one more thing which is do we need to implement all the traits and answer is I don't know
// if there are default methods then leave it as it is othervise compiler will let you know once again.

// Primary trait

// pub trait Summary {
//    fn summarize(&self) -> String;
// }

// trait with default implementation
// pub trait Summary {
//    fn summarize(&self) -> String {
//        String::from("(Read More)")
//    }
// }

// Both Default and Overridden
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// default trait implementation
// impl Summary for NewsArticle {}

// If you don’t implement the method in your impl, Rust uses the default from the trait.
// If you do implement it, you are overriding the default.

// if I want to override the default implementation

// impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
// }

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// impl Summary for SocialPost {
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
// }

// overriding and keeping default both
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters

// pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
// }

// now the thing here is this is a concrete type we don't want that so what do we do, now we will
// generics types.

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple types which implement same trait

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// multiple tarit bounds with + syntax
// pub fn notify(item : &(impl Summary + Display)) {}
// pub fn notify<T : Summary + Display>(item : &[T]) {}

// clearer trait bound with where clause
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t : &T, u : &U) -> i32
// where
//  T : Display + Clone
//  U : Clone + Debug
//  {}

// return types that implement traits
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}
