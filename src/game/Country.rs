use crate::GameMap;

#[derive(Clone)]
pub struct Country {
    name: String,
    population: i64,
    army_size: i64,
    conquered_countries: Vec<String>,
    is_conquered: bool,
}

impl Country {
    pub fn new(name: String, population: i64, army_size: i64, conquered_countries: Vec<String>, is_conquered: bool) -> Self {
        Self {
            name: name.to_string(),
            population: population,
            army_size: army_size,
            conquered_countries: conquered_countries.to_vec(),
            is_conquered: is_conquered,
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_population(&self) -> &i64 {
        return &self.population;
    }

    pub fn get_army_size(&self) -> &i64  {
        return &self.army_size;
    }

    pub fn get_conquered_nations(&self) -> Vec<String> {
        return self.conquered_countries.clone();
    }

    pub fn get_is_conquered(&self) -> bool {
        return self.is_conquered;
    }

    pub fn set_population(&mut self, population: i64) {
        self.population = population;
    }

    pub fn set_army_size(&mut self, army_size: i64) {
        self.army_size = army_size;
    }

    pub fn set_conquered_nations(&mut self, conquered_countries: Vec<String>) {
        self.conquered_countries = conquered_countries
    }

    pub fn set_is_conquered(&mut self, is_conquered: bool) {
        self.is_conquered = is_conquered;
    }
}