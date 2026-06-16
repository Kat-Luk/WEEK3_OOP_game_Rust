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

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_population(&self) -> i64 {
        return self.population;
    }

    pub fn get_army_size(&self) -> i64  {
        return self.army_size;
    }
}