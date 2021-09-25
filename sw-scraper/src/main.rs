mod model;

mod error;
extern crate reqwest;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::{fs::File, io::Write, iter::FromIterator, path::Path};

use crate::{
    error::AppError,
    model::{EntityType, Film, People, Planet, Species, Starship, Url, Vehicle},
};

#[derive(Debug)]
struct NextUrlToFetch {
    url: Option<String>,
    results: Vec<Value>,
}

trait Factor {
    fn factorial_tail_rec(url: NextUrlToFetch) -> Self;
    fn factorial(url: NextUrlToFetch) -> Self;
}

impl Factor for NextUrlToFetch {
    fn factorial_tail_rec(url: NextUrlToFetch) -> Self {
        url
    }

    fn factorial(mut input: NextUrlToFetch) -> Self {
        //fetch the next results
        let current_url_to_fetch = &input.url;

        //check pagination "next", match Some/None
        if let Some(next_url_to_fetch) = current_url_to_fetch {
            println!("next url to fetch!! {:?}", next_url_to_fetch);

            let sr = reqwest::blocking::get(next_url_to_fetch)
                .unwrap()
                .json::<Value>()
                .unwrap();

            let next_page = &sr["next"];

            println!("found results next url: {:?} {:?}", &next_page, sr["count"]);
            sr["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|f| input.results.push(f.to_owned()));

            match next_page {
                Value::String(next_page) => {
                    let u = Self::factorial(NextUrlToFetch {
                        url: Some(next_page.to_string()),
                        ..input
                    });
                    u
                }
                _ => NextUrlToFetch { url: None, ..input },
            }
        } else {
            NextUrlToFetch { url: None, ..input }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Collection<T> {
    results: Vec<T>,
}
impl<Film> Collection<Film> {
    fn new(results: Vec<Film>) -> Self {
        Self { results }
    }

    fn add(&mut self, elem: Film) {
        self.results.push(elem);
    }
}

impl FromIterator<Value> for Collection<Film> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| Film::from(f)).collect())
    }
}
impl FromIterator<Value> for Collection<Planet> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| Planet::from(f)).collect())
    }
}

impl FromIterator<Value> for Collection<Species> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| Species::from(f)).collect())
    }
}

impl FromIterator<Value> for Collection<Vehicle> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| Vehicle::from(f)).collect())
    }
}


impl FromIterator<Value> for Collection<Starship> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| Starship::from(f)).collect())
    }
}
fn main() -> Result<(), AppError> {
    let base_url = "https://api.starwars.run/api";

    // Helper Function - entity type to url
    let to_url = |entity_type: EntityType| -> Url {
        match entity_type {
            _ => format!("{}/{}/", base_url, entity_type),
        }
    };

    //FILMS
    let find_all = || -> Collection<Film> {
        let results = vec![];
        let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
            url: Some(to_url(EntityType::Film)),
            results,
        });
        let y = active_url.results.into_iter().collect::<Collection<Film>>();
        y
    };

    let to_path = |file_name: &str| -> File {
        let p = Path::new(file_name);
        File::create(&p).unwrap()
    };
    let mut file = apply(to_path, "films-new.json");
    let to_bytes = |all: Vec<Film>| -> Result<String, AppError> {
        serde_json::to_string(&all).map_err(|e| AppError {
            message: Some(String::from("failed to serialize data to json")),
            cause: Some(e.to_string()),
            error_type: error::AppErrorType::_InvalidData,
        })
    };
    let content = apply(to_bytes, find_all().results)?;
    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write content to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })?;

    //PLANETS
    let find_all = || -> Collection<Planet> {
        let results = vec![];
        let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
            url: Some(to_url(EntityType::Planet)),
            results,
        });
        let y = active_url
            .results
            .into_iter()
            .collect::<Collection<Planet>>();
        y
    };

    let mut file = apply(to_path, "planets-new.json");
    let to_bytes = |all: Collection<Planet>| -> Result<String, AppError> {
        serde_json::to_string(&all).map_err(|e| AppError {
            message: Some(String::from("failed to serialize data to json")),
            cause: Some(e.to_string()),
            error_type: error::AppErrorType::_InvalidData,
        })
    };
    let content = apply(to_bytes, find_all())?;
    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write content to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })?;

    //SPECIES
    let find_all_species = || -> Collection<Species> {
        let results = vec![];
        let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
            url: Some(to_url(EntityType::Species)),
            results,
        });
        let y = active_url
            .results
            .into_iter()
            .collect::<Collection<Species>>();
        y
    };
    let mut file = apply(to_path, "species-new.json");
    let to_bytes = |all: Collection<Species>| -> Result<String, AppError> {
        serde_json::to_string(&all).map_err(|e| AppError {
            message: Some(String::from("failed to serialize data to json")),
            cause: Some(e.to_string()),
            error_type: error::AppErrorType::_InvalidData,
        })
    };
    let content = apply(to_bytes, find_all_species())?;
    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write content to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })?;

    //VEHICLES
    let find_all_vehicles = || -> Collection<Vehicle> {
        let results = vec![];
        let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
            url: Some(to_url(EntityType::Vehicle)),
            results,
        });
        let y = active_url
            .results
            .into_iter()
            .collect::<Collection<Vehicle>>();
        y
    };
    let mut file = apply(to_path, "vehicle-new.json");
    let to_bytes = |all: Collection<Vehicle>| -> Result<String, AppError> {
        serde_json::to_string(&all).map_err(|e| AppError {
            message: Some(String::from("failed to serialize data to json")),
            cause: Some(e.to_string()),
            error_type: error::AppErrorType::_InvalidData,
        })
    };
    let content = apply(to_bytes, find_all_vehicles())?;
    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write content to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })?;

    //STARSHIPS
    let find_all_starships = || -> Collection<Starship> {
        let results = vec![];
        let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
            url: Some(to_url(EntityType::Starship)),
            results,
        });
        let y = active_url
            .results
            .into_iter()
            .collect::<Collection<Starship>>();
        y
    };
    let mut file = apply(to_path, "starship-new.json");
    let to_bytes = |all: Collection<Starship>| -> Result<String, AppError> {
        serde_json::to_string(&all).map_err(|e| AppError {
            message: Some(String::from("failed to serialize data to json")),
            cause: Some(e.to_string()),
            error_type: error::AppErrorType::_InvalidData,
        })
    };
    let content = apply(to_bytes, find_all_starships())?;
    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write content to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })?;
    Ok(())
}

fn apply<F, A, B>(fun: F, args: A) -> B
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
