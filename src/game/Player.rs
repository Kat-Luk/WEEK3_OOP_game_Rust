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

    pub fn spy(&self) {
        let mut inspected_country: Option<Country> = None;
        println!("| 1) Spy on a country | 0) Exit program |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim();
        match choice {
            "1" => {
                let mut finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
                let mut sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
                let mut norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
                let mut denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);
                let mut game_map = GameMap::new();
                game_map.add_country(denmark);
                game_map.add_country(finland);
                game_map.add_country(norway);
                game_map.add_country(sweden);

                let country = self.get_country();
                let self_name = country.get_name();
                game_map.list_countries();  
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Error reading input.");
                let choice = choice.trim(); 
                match choice {
                    "1" => {
                        if "Denmark" == self_name {
                            println!("You can't spy on your own nation!");
                        } else {
                            println!("Espionage successful.");
                            inspected_country = Some(game_map.get_country_by_index(1));    
                        }
                    }
                    "2" => {
                        if "Finland" == self_name {
                            println!("You can't spy on your own nation!");
                        } else {
                            println!("Espionage successful");
                            inspected_country = Some(game_map.get_country_by_index(2));    
                        }
                    }
                    "3" => {
                        if "Norway" == self_name {
                            println!("You can't spy on your own nation!");
                        } else {
                            println!("Espionage successful");
                            inspected_country = Some(game_map.get_country_by_index(3));    
                        }
                    }
                    "4" => {
                        if "Sweden" == self_name {
                            println!("You can't spy on your own nation!");
                        } else {
                            println!("Espionage successful");
                            inspected_country = Some(game_map.get_country_by_index(4));
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
            "0" => std::process::exit(0),
            _ => {
                println!("Incorect input");
            }

        }
    }
}