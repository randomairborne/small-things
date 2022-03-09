use serde::Deserialize;
use std::fmt;
use std::io::Write;
use text_io::read;

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.country)
    }
}

#[derive(Deserialize, Clone, Debug)]
struct Country {
    country: String,
    population: i128,
}

fn main() {
    let cfg_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("./countries.json"));
    let countries_string = std::fs::read_to_string(&cfg_path).unwrap();
    let countries = serde_json::from_str::<Vec<Country>>(&countries_string).unwrap();
    print!("Target population: ");
    std::io::stdout().flush().unwrap();
    let target_value: i128 = read!();
    let mut closest_countries: Vec<&Country> = vec![];
    for country1 in &countries {
        for country2 in &countries {
            for country3 in &countries {
                let mut closest_countries_total: i128 = 0;
                for country in &closest_countries {
                    closest_countries_total += country.population
                }
                let closest_countries_distance = target_value - closest_countries_total;
                let off_by: i128 = target_value
                    - (country1.population + country2.population + country3.population);
                if (off_by < closest_countries_distance) && (off_by > 0) {
                    closest_countries = std::vec::Vec::from([country1, country2, country3])
                }
            }
        }
    }
    let mut closest_countries_total: i128 = 0;
    for country in &closest_countries {
        closest_countries_total += country.population
    }
    let closest_countries_distance = target_value - closest_countries_total;
    let mut closest_countries_formatted = String::from("");
    for country in closest_countries {
        closest_countries_formatted = closest_countries_formatted + &country.country + ", ";
    }
    println!(
        "{}Distance: {}",
        closest_countries_formatted, closest_countries_distance
    );
}
