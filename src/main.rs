pub mod game;
use crate::game::Player::*;
use crate::game::Country::*;
use crate::game::GameMap::*;
use std::io::*;
use std::io;
fn main() {
    let mut player;
    let mut game_map = GameMap::new();

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    println!("Choose your country: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input.");
    let choice = choice.trim();
    match choice {
        "1" => player = Player::new(game_map.get_country_by_index(1).clone()),
        "2" => player = Player::new(game_map.get_country_by_index(3).clone()),
        "3" => player = Player::new(game_map.get_country_by_index(2).clone()),
        "4" => player = Player::new(game_map.get_country_by_index(0).clone()),
        _ => {
            println!("Wrong input");
            return;
        }
    }
    loop {
        player.inspect();

        println!("| 1) Spy on a country | 2) Invade a country | 3) Expand military |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim(); 

        match choice {
            "1" => {
                game_map.list_countries();  
                player.spy(&game_map);
            }
            "2" => {
                game_map.list_countries();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Error reading input.");
                let choice = choice.trim(); 
                match choice {
                    "1" => {
                        player.conquer_nation(game_map.get_country_by_index(0).clone(), "Denmark".to_string(),&mut game_map);
                    }
                    "2" => {
                        player.conquer_nation(game_map.get_country_by_index(1).clone(), "Finland".to_string(), &mut game_map);

                    }
                    "3" => {
                        player.conquer_nation(game_map.get_country_by_index(2).clone(), "Norway".to_string(), &mut game_map);
                    }
                    "4" => {
                        player.conquer_nation(game_map.get_country_by_index(3).clone(), "Sweden".to_string(), &mut game_map);
                    }
                    _ => println!("Incorect input"),
                    }
            }
            "3" => {
                let my_name = player.get_country().get_name().clone();
                let mut all_countries = game_map.get_countries().clone();
                for country in all_countries.iter_mut() {
                    if country.get_name() == &my_name {
                        country.expand_army();
                    }
                }
                game_map.set_countries(all_countries);
                player.inspect();
            }
            _ => {
                println!("Incorect input");
            }
        }
    game_map.other_countries_turn(&player.get_country().get_name().clone());
    if player.get_country().get_conquered_nations().len() == 3 {
        println!("You have conquered all your targets. Good work, comrade!");
        break;
    }
    }
}