// You can only implement a trait for a type if you own the trait OR the type (or both).
// For Ex : if I denife a try to use a trait like display on a vec! than i can not use this because
// i do not know any of these that means these both comes from standard library so I can not use
// this and also I do not have these in scope so I can use whatever I have in Scope.

use summary::{SocialPost, Summary, notify, returns_summarizable};
// use summary::{NewsArticle, Summary};

fn main() {
    // Using Trait

    //    let post = SocialPost {
    //        username: String::from("horse_ebooks"),
    //        content: String::from("of course, as you already know, people"),
    //        reply: false,
    //        repost: false,
    //    };
    //    println!("1 new social post  : {}", post.summarize());

    // using default trait

    // let article = NewsArticle {
    //    headline: String::from("Penguins win Stanley Cup Championship!"),
    //    location: String::from("Pittsburgh, PA, USA"),
    //    author: String::from("Iceburgh"),
    //    content: String::from(
    //        "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
    //    ),
    //};
    // println!("New article available! {}", article.summarize());

    // using both

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());

    // using trait as parameters and trait bound
    notify(&post);

    // using return types that implement traits
    let post2 = returns_summarizable();
    println!("1 new social post: {}", post2.summarize());
}
