use std::io;
use crate::Country;
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
                let army = country.get_army_size();
                println!("An inspection has been completed..");    
                println!("Country information:");    
                println!("{}", name);
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
}