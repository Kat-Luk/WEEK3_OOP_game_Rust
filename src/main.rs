pub mod game;
use crate::game::Player::*;
use crate::game::Country::*;
use std::io::*;
use std::io;
fn main() {
    let mut finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
    let mut sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
    let mut norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
    let mut denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    println!("Choose your country: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input.");
    let choice = choice.trim();
    match choice {
        "1" => {
            let player = Player::new(finland);
            player.inspect();
        }
        "2" => {
            let player = Player::new(sweden);
            player.inspect();
        }
        "3" => {
            let player = Player::new(norway);
            player.inspect();
        }
        "4" => {
            let player = Player::new(denmark);
            player.inspect();
        }
        _ => {
            println!("Wrong input");
        }
    }
}
