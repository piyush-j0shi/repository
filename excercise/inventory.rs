use std::io;

struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

impl Item {
    fn total_value(&mut self) -> (u32, f64) {
        let total = self.price * self.quantity as f64;
        (self.quantity, total)
    }

    fn restock(&mut self, quantity: u32) -> u32 {
        self.quantity += quantity;
        self.quantity
    }

    fn sell(&mut self, quantity: u32, price: f64) -> (u32, f64) {
        if self.quantity >= quantity {
            self.quantity -= quantity;
            let revenue = quantity as f64 * price;
            self.price += revenue;

            (self.quantity, self.price)
        } else {
            println!("not enough quantity available");
            (self.quantity, 0.0)
        }
    }
}

fn main() {
    let mut inventory = Item {
        name: String::from("name"),
        price: 2000.0,
        quantity: 10,
    };

    println!("\n === before restocking === \n");
    let (quant, amnt) = inventory.total_value();

    println!("item name : {}", inventory.name);
    println!("current quantity : {}", quant);
    println!("current amount : {}", amnt);

    println!("\n === new cycle === \n");

    println!("enter the quantity you want to restock : ");

    let mut operational_amount = String::new();

    io::stdin()
        .read_line(&mut operational_amount)
        .expect("Failed to read line");

    let operational: u32 = operational_amount
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let new_quantuty: u32 = inventory.restock(operational);

    println!("\n === after restocking === \n");
    println!("quantity after restock : {}", new_quantuty);

    println!("\n === new cycle === \n");

    println!("enter the quantity you want to sell : ");
    let mut sell_quantity = String::new();

    io::stdin()
        .read_line(&mut sell_quantity)
        .expect("Failed to read line");

    let sell_quantity: u32 = sell_quantity
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("enter the price per quantity : ");
    let mut sell_amount = String::new();

    io::stdin()
        .read_line(&mut sell_amount)
        .expect("Failed to read line");

    let sell_amount: f64 = sell_amount
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let (after_quantity, after_amount) = inventory.sell(sell_quantity, sell_amount);
    println!("quantity after sell : {}", after_quantity);
    println!("amount after sell : {}", after_amount);
}
