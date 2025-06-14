use std::io;

#[derive(Debug)]

struct Account {
    owner: String,
    balance: f64,
}

// predefined input

// impl Account {
//     fn deposite(&mut self, amount: f64) {
//         if amount < 0.0 {
//             println!("amount should be greater than zero");
//         } else {
//             self.balance += amount;
//             println!("total balance : {}", self.balance);
//         }
//     }

//     fn withdrawal(&mut self, withdraw_amount: f64) {
//         if self.balance < 0.0 {
//             println!("current balance should be greater than zero");
//         } else {
//             self.balance -= withdraw_amount;
//             println!("current balance is : {}", self.balance)
//         }
//     }

//     fn get_balance(&self) {
//         println!("current balance is : {}", self.balance)
//     }
// }

impl Account {
    fn deposite(&mut self, amount: f64) {
        if amount < 0.0 {
            println!("amount should be greater than zero");
        } else {
            self.balance += amount;
            // println!("total balance : {}", self.balance);
        }
    }

    fn withdrawal(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("Withdrawal amount should be greater than zero.");
        } else if amount > self.balance {
            println!("Insufficient funds. Current balance: ₹{:.2}", self.balance);
        } else {
            self.balance -= amount
            //  println!("Withdrew ₹{:.2}. New balance: ₹{:.2}", amount, self.balance);
        }
    }

    fn get_balance(&self) -> f64 {
        // println!("current balance is : {}", self.balance)
        self.balance
    }
}

fn main() {
    let mut status = Account {
        owner: String::from("Owner"),
        balance: 2500.00,
    };

    println!("enter the operational amount");

    let mut operational_amount = String::new();

    io::stdin()
        .read_line(&mut operational_amount)
        .expect("Failed to read line");

    let operational: f64 = operational_amount
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("\n== Deposit ==");
    status.deposite(operational);
    println!("Final Balance: ₹{:.2}", status.get_balance());

    println!("\n== Withdrawal ==");
    status.withdrawal(operational);
    println!("Final Balance: ₹{:.2}", status.get_balance());

    println!("\n== Final Account Summary ==");
    println!("Owner: {}", status.owner);
    println!("Final Balance: ₹{:.2}", status.get_balance());
}
