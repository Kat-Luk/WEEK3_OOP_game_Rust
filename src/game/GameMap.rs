use crate::Country;

pub struct GameMap {
    countries: Vec<Country>,
}

impl GameMap {
    pub fn new(countries: Vec<Country>) -> Self {
        Self {
            countries: countries.to_vec(),
        }
    }

    pub fn list_countries(&self) {
        println!("1) {}", self.countries[0].get_name());
        println!("2) {}", self.countries[1].get_name());
        println!("3) {}", self.countries[2].get_name());
        println!("4) {}", self.countries[3].get_name());
    }

    pub fn get_country_by_index(&self, index: &str) -> Country {
        match index {
            "1" => return self.countries[0].clone(),
            "2" => return self.countries[1].clone(),
            "3" => return self.countries[2].clone(),
            "4" => return self.countries[3].clone(),
            _ => panic!("Invalid index"),
        }
    }
}