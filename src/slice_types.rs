fn main(){
	let mut s = String::from("hello world");
	let word = first_word(&s);
	println!("the index of first space is : {word}", );
	s.clear();

	let new_s = String::from("hello world");
	let hello = &s[0..5];
	let world = &s[6..11];

	println!("new string : {new_s}, first 5 words : {hello}, second five words : {world}", );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
