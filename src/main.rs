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
}
