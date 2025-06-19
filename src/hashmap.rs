use std::collections::HashMap;

fn main() {
    // declaration
    let mut scores = HashMap::new();

    // inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores : {:?}", scores);

    // accessing values
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("team : {:?}, score : {:?}", team_name, team_score);

    // iteration
    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    // hash map and ownership
    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("name , value : {:?}", map);

    // updating a hash map
    // 1. overwriting a value

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("blue"), 10);
    println!("before overwriting : {:?}", scores1);

    scores1.insert(String::from("blue"), 25);
    println!("after overwriting : {:?}", scores1);

    // 2. adding key and value if a key is not present
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("blue"), 10);

    scores2.entry(String::from("yellow")).or_insert(50);
    scores2.entry(String::from("blue")).or_insert(50);

    println!("{scores2:?}");

    // updating a value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
