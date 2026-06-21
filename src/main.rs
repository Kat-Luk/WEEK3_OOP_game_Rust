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
                
                let target_index: Option<usize> = match choice {
                    "1" => Some(0),
                    "2" => Some(1),
                    "3" => Some(2),
                    "4" => Some(3),
                    _ => {
                        println!("Incorect input");
                        None
                    }
                };
                
                if let Some(index) = target_index {
                    let target = game_map.get_country_by_index(index);
                    let player_army = player.get_country().get_army_size().clone();
                    let target_army = target.get_army_size().clone();
                    let target_name = target.get_name().clone();
                    let already_conquered = player.get_country().get_conquered_nations().contains(&target_name);
                    let is_self = player.get_country().get_name() == &target_name;
                    
                    player.conquer_nation(target.clone(), target_name.clone());
                    
                    if !already_conquered && !is_self && player_army > target_army {
                        let mut all_countries = game_map.get_countries().clone();
                        all_countries[index].set_is_conquered(true);
                        game_map.set_countries(all_countries);
                    }
                }
            }
            "3" => {
                let my_name = player.get_country().get_name().clone();
                let mut all_countries = game_map.get_countries().clone();
                for country in all_countries.iter_mut() {
                    if country.get_name() == &my_name {
                        country.expand_military();
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
