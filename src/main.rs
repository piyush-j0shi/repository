#[allow(dead_code)]
// every reference in rust has a lifetime, which is the scope for which that reference is valid

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// function with lifetimes
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // here we are denoting the lifetime of r with 'a and x with 'b so just to thing the r has
    // the inner 'b block is much smaller than the outer 'a lifetime block. At compile time,
    // Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that
    // it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter
    // than 'a: the subject of the reference doesn’t live as long as the reference so the code
    // won't work.

    // let r;                // ---------+-- 'a
    //          |
    // {                     //          |
    //  let x = 5;           // -+-- 'b  |
    //  r = &x;              //  |       |
    // }                     // -+       |
    //          |
    // println!("r: {r}");   //          |
    //                       // ---------+

    // But here the code will work because the lifetime 'b is greater than 'a, because rust knows
    // that the reference of r will always be valid in x.

    // let x = 5;            // ----------+-- 'b
    //                       //           |
    // let r = &x;           // --+-- 'a  |
    //                       //   |       |
    // println!("r: {r}");   //   |       |
    //                       // --+       |
    //                       //    ----------+

    // same goes here rust needs to know that the reference of r will always be valid in x

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("the longest string is : {result}")

    // lifetime annotations
    // &i32 // a reference
    // &'a i32 // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // using the function here

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("the longest string is : {result}");

    // this is valid

    // let string1 = String::from("long string is long");
    // {
    //     let string2 = String::from("abcd");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("the longest string is : {result}")
    // }

    // this is not valid because when we use lifetimes rust takes the shortest overlapping lifetime and due to
    // this when we try to use the returned reference when one of the argument goes out of scope
    // then it causes the error because the lifetime has ended.

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // Lifetime annotations in struct definitions

    let novel = String::from("call me someone. some years ago.....");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt is : {:?}", i);
}
