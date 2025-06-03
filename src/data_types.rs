fn main() {
	let guess: u32 = "42".parse().expect("not a number");
	println!("the guess is : {guess}");

	let x = 2.0;
	let y = 3.0;
	let sum = x / y;
	println!("the result of division is : {sum}");

	let sum = 5 + 10;
	println!("the result of sum is : {sum}", );

	let difference_positive = 15 - 10;
	let difference_negative = 10 - 15;
	println!("postive and negative difference is : {difference_positive}, {difference_negative}", );

	let multiplication = 15 * 10;
	println!("the result of multiplication is : {multiplication}");

	let division_float = 10.0 / 5.5;
	let division_int = 10 / 5;
	println!("division_int and division_float is : {division_int}, {division_float}");

	let reminder = 10 / 3;
	println!("the result of reminder is : {reminder}");

	let t = true;
	let f : bool = false;
	println!("without type and with type : {f}, {t}");

	let tup : (i32, f64, u32) = (3, 3.5, 3);
	println!("the tuple is : {:?}", tup);

	let first_value = tup.0;
	let second_value = tup.1;
	println!("first and second value is : {first_value}, {second_value}", );

	let arrayy = [1, 2, 3, 4, 5];
	println!("the array is : {:?}", arrayy);

	let arrayy_size = [3;5];
	println!("the sized array is : {:?}", arrayy_size);

	let arrayy_difelement : [i32; 5] = [1, 2, 3, 4, 5];
	println!("the arrayy_difelement is : {:?}", arrayy_difelement);
}
