use crate::Country;

pub struct GameMap {
    countries: Vec<Country>,
}

impl GameMap {
    pub fn new() -> Self {
        Self {
            countries: vec![
                Country::new(String::from("Denmark"), 6000000, 50000, vec![], false),
                Country::new(String::from("Finland"), 5600000, 900000, vec![], false),
                Country::new(String::from("Norway"), 5500000, 100000, vec![], false),
                Country::new(String::from("Sweden"), 10000000, 200000, vec![], false),
            ],
        }
    }

    pub fn list_countries(&self) {
        println!("1) {}", self.countries[0].get_name());
        println!("2) {}", self.countries[1].get_name());
        println!("3) {}", self.countries[2].get_name());
        println!("4) {}", self.countries[3].get_name());
    }

    pub fn get_country_by_index(&self, index: usize) -> Country {
        self.countries[index].clone()
    }

    pub fn get_countries(&self) -> &Vec<Country> {
        &self.countries
    }

    pub fn set_countries(&mut self, countries: Vec<Country>) {
        self.countries = countries;
    }

    pub fn other_countries_turn(&mut self, my_name: &String) {
        let mut all_countries = self.get_countries().clone();
        for country in all_countries.iter_mut() {
            if country.get_name() != my_name && !country.get_is_conquered() {
                country.expand_army();
            }
        }
        self.set_countries(all_countries);
    }
}