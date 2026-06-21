use std::io;
use crate::Country;
use crate::GameMap;

pub struct Player {
    country: Country,
}

impl Player {
    pub fn new(country: Country) -> Self {
        Self {
            country: country,
        }
    }
    pub fn get_country(&self) -> &Country{
        return &self.country;
    }
    pub fn inspect(&self) {
        println!("| Inspection on your own nation? | y = yes | n = no |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim();

        match choice {
            "y" => {
                let country = self.get_country();
                let name = country.get_name();
                let population = country.get_population();
                println!("An inspection has been completed..");
                let army = country.get_army_size();
                    
                println!("Country information:");    
                println!("Name: {}", name);
                println!("Population: {}", population);
                println!("Army size: {}", army);
            }
            "n" => {
                println!("The leader is confident. No inspection needed.");
            }
            _ => {
                println!("Incorect input")
            }
        }
    }

    pub fn spy(&self, game_map: &GameMap) {
        let mut inspected_country: Option<Country> = None;
        let player_country = self.get_country(); 
        let my_name = player_country.get_name();
        for i in 0..4 {
            if game_map.get_country_by_index(i).get_name() == my_name {
                let my_country_game_map = game_map.get_country_by_index(i);
            }  
        } 
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim(); 
        match choice {
            "1" => {
                if "Denmark" == my_name {
                    println!("You can't spy on your own nation!");
                } else {
                    println!("Espionage successful.");
                    inspected_country = Some(game_map.get_country_by_index(0));    
                }
            }
            "2" => {
                if "Finland" == my_name {
                    println!("You can't spy on your own nation!");
                } else {
                    println!("Espionage successful.");
                    inspected_country = Some(game_map.get_country_by_index(1));    
                }
            }
            "3" => {
                if "Norway" == my_name {
                    println!("You can't spy on your own nation!");
                } else {
                    println!("Espionage successful.");
                    inspected_country = Some(game_map.get_country_by_index(2));    
                }
            }
            "4" => {
                if "Sweden" == my_name {
                    println!("You can't spy on your own nation!");
                } else {
                    println!("Espionage successful.");
                    inspected_country = Some(game_map.get_country_by_index(3));
                }
            }
            _ => println!("Incorect input"),
        }
        if let Some(country) = inspected_country {
            let name = country.get_name();
            let population = country.get_population();
            let army = country.get_army_size();
            println!("Country information:");
            println!("Name: {}", name);
            println!("Population: {}", population);
            println!("Army size: {}", army);  
        }
    }

    pub fn conquer_nation(&mut self, target_country: Country, country_name: String) {
        println!("DEBUG conquer_nation called: target={}, army_mine={}, army_theirs={}", target_country.get_name(), self.country.get_army_size(), target_country.get_army_size());
        let my_name = self.country.get_name().clone();
        let target_name = target_country.get_name().clone();
        let already_conquered = self.country.get_conquered_nations().contains(&target_name);

        if already_conquered {
            println!("This land is already conquered.");
        } else if my_name == target_name {
            println!("Even your sick desires need boundaries.");
        } else if self.country.get_army_size() > target_country.get_army_size() {
            let mut conquered = self.country.get_conquered_nations();
            conquered.push(target_name.clone());
            self.country.set_conquered_nations(conquered);

            let new_population = self.country.get_population() + target_country.get_population();
            let new_army = self.country.get_army_size() + target_country.get_army_size();
            self.country.set_population(new_population);
            self.country.set_army_size(new_army);

            println!("You have conquered {}", target_name);
        } else if self.country.get_army_size() == target_country.get_army_size() {
            println!("The armies are equally matched. Neither side gains ground.");
        } else {
            println!("You have lost your war against {}. You have been conquered.", target_name);
            println!("Game over!");
            std::process::exit(0);
        }
    }
}