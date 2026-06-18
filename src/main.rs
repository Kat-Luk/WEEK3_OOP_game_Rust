pub mod game;
use crate::game::Player::*;
use crate::game::Country::*;
use crate::game::GameMap::*;
use std::io::*;
use std::io;
fn main() {
    
    let player;
    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    println!("Choose your country: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input.");
    let choice = choice.trim();
    match choice {
        "1" => player = Player::new(finland),
        "2" => player = Player::new(sweden),
        "3" => player = Player::new(norway),
        "4" => player = Player::new(denmark),
        _ => {
            println!("Wrong input");
            return;
        }
    }
    loop {
        player.inspect();
        player.spy();
    }
}
