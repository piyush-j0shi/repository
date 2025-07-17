#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user1_preference = Some(ShirtColor::Red);
    let user2_preference = None;

    let giveaway1 = inventory.giveaway(user1_preference);
    let giveaway2 = inventory.giveaway(user2_preference);

    println!(
        "User 1 gets: {:?} with preference : {:?}",
        giveaway1, user1_preference
    );
    println!(
        "User 2 gets: {:?} with preference : {:?}",
        giveaway2, user2_preference
    );
}
