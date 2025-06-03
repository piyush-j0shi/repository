fn main(){
	let s= String::from("hello");
	take_ownership(s);
	let x = 5;
	makes_copy(x);
	println!("{}", x);

	let s_2 = gives_ownership();
	println!("s_2 is {s_2}", );

	let s_3 = String::from("hello");
	println!("s_3 after String::from : {s_3}");

	let s_4 = takes_and_gives_backs(s_3);
	println!("s_4 is : {s_4}");
} 

fn take_ownership(some_string : String){
	println!("{some_string}");
}

fn makes_copy(some_integer : i32){
	println!("{some_integer}");
}

fn gives_ownership() -> String {
	let some_string_1 = String::from("yours");
	some_string_1
}

fn takes_and_gives_backs(some_string_2 : String) -> String{
	some_string_2
}