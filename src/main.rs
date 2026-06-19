pub mod game;
use crate::game::Player::*;
use crate::game::Country::*;
use crate::game::GameMap::*;
use std::io::*;
use std::io;
fn main() {
    let mut player;
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
        "1" => player = Player::new(finland.clone()),
        "2" => player = Player::new(sweden.clone()),
        "3" => player = Player::new(norway.clone()),
        "4" => player = Player::new(denmark.clone()),
        _ => {
            println!("Wrong input");
            return;
        }
    }
    loop {
        player.inspect();

        println!("| 1) Spy on a country | 2) Invade a country | 0) Exit program |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim(); 

        match choice {
            "1" => {
                player.spy();
            }
            "2" => {
                let mut game_map = GameMap::new();
                game_map.list_countries();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Error reading input.");
                let choice = choice.trim(); 
                match choice {
                    "1" => {
                        player.conquer_nation(denmark.clone(), "Denmark".to_string());
                    }
                    "2" => {
                        player.conquer_nation(finland.clone(), "Finland".to_string());

                    }
                    "3" => {
                        player.conquer_nation(norway.clone(), "Norway".to_string());
                    }
                    "4" => {
                        player.conquer_nation(sweden.clone(), "Sweden".to_string());
                    }
                    _ => println!("Incorect input"),
                    }
            }
            "0" => std::process::exit(0),
            _ => {
                println!("Incorect input");
            }
        }
    }
}
