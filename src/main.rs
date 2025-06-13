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
}
