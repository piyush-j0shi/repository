use std::io;

fn main(){
	println!("enter a number");
	let mut number = String::new();
	io::stdin().read_line(&mut number).expect("failed to read the value");
	let number : i32 = number.trim().parse().expect("not a number");

	if number == 0{
		println!("number should be greater than zero");
		return;
	} 

	else if number % 2 == 0 {
		println!("{number} is even");
	}

	else {
		println!("{number} is odd");
	}

	// using if in a let statement

	let condition = false;
	let numb = if condition {5} else {6};
	println!("the value of number is : {numb}", );

	// loop in rust

	// loop{
	// 	println!("again");
	// }

	// returning values from loop

	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};

	println!("the result is : {result}");

	// Multiple loops 

	let mut count = 0;
	'counting_up : loop{
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");

			if remaining == 9{
				break;
			}

			if count == 2{
				break 'counting_up;
			}

			remaining -= 1;
		}

		count += 1;
	}

	println!("end count = {count}");

	// While loop

	let mut num = 3;
	while num != 0{
		println!("{num}");
		num -= 1;
	}

	println!("LIFTOFF!!!");

	// for loop
	let collection = [10, 20, 30, 40, 50];

	for elements in collection{
		println!("the value is : {elements}", );
	}

	// range in for loop
	for number in (1..4).rev(){
		println!("{number}", );
	}
	println!("LIFTOFF!!!");
}