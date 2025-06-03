#[derive(Debug)]

struct Rectangle {
	width : u32,
	height : u32
}

fn main() {
	// let width = 30;
	// let height = 50;

	// let rect1 = (30, 50);

	let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // println!("rect1 is {:#?}", rect1); //pretty-print {:#?}

	println!(
		"area of rectange is {} square pixels",
		area(&rect1)
	);
}

// Common way 

// fn area(width : u32, height : u32) -> u32{
// 	width * height
// }

// Using tuple struct

// fn area(dimension : (u32, u32)) -> u32{
// 	dimension.0 * dimension.1
// }

fn area(rectangle : &Rectangle) -> u32{
	rectangle.width * rectangle.height
}