fn main() {
    // mutable and immutable

    let mut x = 5;
    println!("the value of x is : {x}");

    x = 6;
    println!("the value of x is : {x}");

    // const
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("three hours in seconds : {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("the value of y inside the scope is : {y}");
    }

    println!("the value of y outside the scope is : {y}");

    // mut vs shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("the length of space is : {spaces}");

    let mut spaces_1 = "    ";
    spaces_1 = spaces_1.len();
    println!("the length of spaces_1 is :{spaces_1}", );
}
