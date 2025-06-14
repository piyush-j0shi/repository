use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("(for debug) secret_number : {:?}", secret_number);
    loop {
        println!("please input your guess : ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        println!("your guess : {input}");
        let input: u32 = input.trim().parse().expect("failed to read number");

        match input.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),

            Ordering::Equal => {
                println!("over");
                break;
            }
        }
    }
}
