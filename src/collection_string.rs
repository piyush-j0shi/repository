fn main() {
    // let mut s = String::new();

    let data = "initial content";
    let s = data.to_string();

    println!("the string is : {s}");

    // we can also write
    // let s = "initial content".to_string();

    // OR

    // let s = String::from("initial content");

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שלום");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // appending and updating a string
    let mut s1 = String::from("foo");
    println!("old string is : {s1}");

    s1.push_str("bar");
    println!("updated string is : {s1}");

    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(s3);
    println!("the new string is : {s2}");

    let mut s4 = String::from("lo");
    println!("old string is : {s4}");

    s4.push('l');
    println!("updated string is : {s4}");

    // concatenation is string

    let s5 = String::from("hello ");
    let s6 = String::from("world");
    let s7 = s5 + &s6;
    println!("concatenated string is : {s7}");

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");

    // let s11 = s8 + "-" + &s9 + "-" + &s10;
    // println!("the new concatenated string is : {s11}");

    // another way to concatenate is
    let s12 = format!("{s8}-{s9}-{s10}");
    println!("other way of concatenation is : {s12}");
}
