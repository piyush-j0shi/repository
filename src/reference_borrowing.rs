fn main() {
    let s1 = String::from("hello");
    println!("string s is : {s1}");

    let len = calculate_length(&s1);
    println!("length of string is : {len} and string is {s1}");

    // let len_1 = calculate_length_ownership(s1);
    // println!("length of string is : {len} and string is {s1}");
    // This function takes ownership bcz we are not using reference

    let mut s1 = String::from("hello");
    change_value(&mut s1);
    println!("s1 is : {:?}", { s1 });
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn calculate_length_ownership(s : String) -> usize{
// 	s.len()
// }

fn change_value(some_string: &mut String) {
    some_string.push_str(" world")
}
