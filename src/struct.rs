#[derive(Debug)]

struct User {
	active : bool,
	username : String,
	email : String,
	sing_in_count : u64
}

struct Color(i32, i32, i32);

fn main(){
	let user1 = User{
		active : true,
		username : String::from("username123"),
		email : String::from("email@email.com"),
		sing_in_count : 1
	};

	println!("user email is : {:?}", {user1.email});

	let mut user2 = User{
		active : true,
		username : String::from("username456"),
		email : String::from("email@email.com"),
		sing_in_count : 1
	};

	println!("user2 username before modification : {:?}", user2.username);
	user2.username = String::from("username789");
	println!("user2 username after modification : {:?}", user2.username);

	let user3 = User{
		active : user1.active,
		username : user1.username,
		email : String::from("anotheremail@email.com"),
		sing_in_count : user1.sing_in_count
	};

	//we can also define like this if we have only one field 

	// let user3 = User{
	// 	email : String::from("anotheremail@email.com"),
	// 	..user1
	// }

	let black = Color(0, 0, 0);
}

fn build_user(email : String, username : String) -> User {
	User{
		active : true,
		username,
		email,
		sing_in_count : 1
	}
}