mod model;

mod error;
extern crate reqwest;

use serde_json::Value;
use std::{fs::File, io::Write, path::Path};

use crate::{
    error::AppError,
    model::{EntityType, Film, People, Planet, Species, Starship, Url, Vehicle},
};
fn main() -> Result<(), AppError> {
    let base_url = "https://api.starwars.run/api";

    let entity_to_url = |entity_type: EntityType| -> Url {
        match entity_type {
            _ => format!("{}/{}/", base_url, entity_type),
        }
    };
    let find_all_films = || -> Vec<Film> {
        let mut results: Vec<Film> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::Film));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(Film::from(f)));
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };

    let all_films = find_all_films();
    //WRITE TO FILE
    let path = Path::new("films.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_films).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let find_all_planets = || -> Vec<Planet> {
        let mut results: Vec<Planet> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::Planet));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(Planet::from(f)));
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };

    let all_planets = find_all_planets();
    //WRITE TO FILE
    let path = Path::new("planets.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_planets).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let find_all_starships = || -> Vec<Starship> {
        let mut results: Vec<Starship> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::Starship));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(Starship::from(f)));
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };

    let all_starships = find_all_starships();
    //WRITE TO FILE
    let path = Path::new("starships.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_starships).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let find_all_vehicles = || -> Vec<Vehicle> {
        let mut results: Vec<Vehicle> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::Vehicle));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(Vehicle::from(f)));
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };

    let all_vehicles = find_all_vehicles();
    //WRITE TO FILE
    let path = Path::new("vehicles.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_vehicles).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let find_all_species = || -> Vec<Species> {
        let mut results: Vec<Species> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::Species));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(Species::from(f)));
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };

    let all_species = find_all_species();
    //WRITE TO FILE
    let path = Path::new("species.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_species).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let find_all_peeps = || -> Vec<People> {
        let mut results: Vec<People> = vec![];
        let mut active_url = Some(entity_to_url(EntityType::People));
        while let Some(next_url) = active_url {
            println!("fetching url: {:?}", next_url);

            let sr = reqwest::blocking::get(next_url.clone())
                .unwrap()
                .json::<Value>()
                .unwrap();
            println!("found results next url: {:?} {:?}", sr["next"], sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| results.push(People::from(f)));
            //  add_people_result(&mut sr.results);
            let next = &sr["next"];
            match next {
                Value::Null => active_url = None,
                Value::String(next) => active_url = Some(next.to_string()),
                _ => todo!(),
            };
        }
        results
    };
    let all_people = find_all_peeps();

    //WRITE TO FILE
    let path = Path::new("people.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();

    match file.write_all(serde_json::to_string(&all_people).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    Ok(())
}

fn _apply<F, A, B>(fun: F, args: A) -> B
where
    F: Fn(A) -> B,
{
    fun(args)
}

fn compose<X, Y, Z, F, G>(f: F, g: G) -> impl Fn(X) -> Z
where
    F: Fn(X) -> Y,
    G: Fn(Y) -> Z,
{
    move |x| g(f(x))
}
