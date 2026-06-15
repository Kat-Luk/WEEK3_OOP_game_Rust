pub mod game;
use crate::game::Country::*;
use std::io::*;
use std::io;
fn main() {
    let mut finland = Country::new("Finland", &5600000, &900000, &vec![], &false);
    let mut sweden = Country::new("Sweden", &10000000, &200000, &vec![], &false);
    let mut norway = Country::new("Norway", &5500000, &100000, &vec![], &false);
    let mut denmark = Country::new("Denmark", &6000000, &50000, &vec![], &false);

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input.");
    let choice = choice.trim();
    println!("Choose your country: ");
    match choice {
        "1" => {
            let name = finland.get_name();
            let population = finland.get_population();
            let army = finland.get_army_size();
            println!("Country: {}", name);
            println!("Population: {}", population);
            println!("Army size: {}", army);
        }
        "2" => {
            let name = sweden.get_name();
            let population = sweden.get_population();
            let army = sweden.get_army_size();
            println!("Country: {}", name);
            println!("Population: {}", population);
            println!("Army size: {}", army);
        }
        "3" => {
            let name = norway.get_name();
            let population = norway.get_population();
            let army = norway.get_army_size();
            println!("Country: {}", name);
            println!("Population: {}", population);
            println!("Army size: {}", army);
        }
        "4" => {
            let name = denmark.get_name();
            let population = denmark.get_population();
            let army = denmark.get_army_size();
            println!("Country: {}", name);
            println!("Population: {}", population);
            println!("Army size: {}", army);
        }
        _ => {
            println!("Wrong input");
        }
    }
}
