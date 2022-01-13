mod error;
mod model;

extern crate reqwest;

use model::Collection;
use serde::Serialize;

use serde_json::Value;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use tokio::join;

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
static BASE_URL: &'static str = "https://api.starwars.run/api";

fn write_to_file<T>(file_name: &'static str, f: impl Fn() -> Collection<T>) -> Result<(), AppError>
where
    T: Serialize,
{
    let mut file = get_file(file_name)?;
    let content = apply(to_bytes, f())?;

    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write all to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::WriteError,
    })
}

fn get_file(file_name: &'static str) -> Result<File, AppError> {
    apply(to_path, file_name).map_err(|e| AppError {
        message: Some(String::from("failed to create file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::WriteError,
    })
}

fn fetch_all_pages(entity: EntityType) -> Vec<Value> {
    let results = vec![];
    let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
        url: Some(to_url(entity)),
        results,
    });

    active_url.results
}

fn to_url(entity_type: EntityType) -> Url {
    match entity_type {
        _ => format!("{}/{}/", BASE_URL, entity_type),
    }
}

fn to_path(file_name: &'static str) -> Result<File, std::io::Error> {
    File::create(Path::new(file_name))
}

fn to_bytes<T>(all: Collection<T>) -> Result<String, AppError>
where
    T: Serialize,
{
    serde_json::to_string(&all).map_err(|e| AppError {
        message: Some(String::from("failed to serialize data to json")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::_InvalidData,
    })
}

#[tokio::main]
async fn main() {
    //create base output dir
    fs::create_dir::<_>("output").unwrap();
    let mut handles = vec![];
    handles.push(tokio::spawn(async move {
        //Film
        let find_all = || {
            fetch_all_pages(EntityType::Film)
                .into_iter()
                .collect::<Collection<Film>>()
        };
        write_to_file("output/Film.json", find_all).unwrap()
    }));
    handles.push(tokio::spawn(async move {
        //Planet
        let find_all = || {
            fetch_all_pages(EntityType::Planet)
                .into_iter()
                .collect::<Collection<Planet>>()
        };
        write_to_file("output/Planet.json", find_all).unwrap()
    }));
    handles.push(tokio::spawn(async move {
        //People
        let find_all = || {
            fetch_all_pages(EntityType::People)
                .into_iter()
                .collect::<Collection<People>>()
        };
        write_to_file("output/People.json", find_all).unwrap()
    }));
    handles.push(tokio::spawn(async move {
        //Species
        let find_all = || {
            fetch_all_pages(EntityType::Species)
                .into_iter()
                .collect::<Collection<Species>>()
        };
        write_to_file("output/Species.json", find_all).unwrap()
    }));
    handles.push(tokio::spawn(async move {
        //Starship
        let find_all = || {
            fetch_all_pages(EntityType::Starship)
                .into_iter()
                .collect::<Collection<Starship>>()
        };
        write_to_file("output/Starship.json", find_all).unwrap()
    }));

    handles.push(tokio::spawn(async move {
        //Vehicle
        let find_all = || {
            fetch_all_pages(EntityType::Vehicle)
                .into_iter()
                .collect::<Collection<Vehicle>>()
        };
        write_to_file("output/Vehicle.json", find_all).unwrap()
    }));

    let joins = join!(
        handles.pop().unwrap(),
        handles.pop().unwrap(),
        handles.pop().unwrap(),
        handles.pop().unwrap(),
        handles.pop().unwrap(),
        handles.pop().unwrap()
    );
    joins.0.unwrap();
    joins.1.unwrap();
    joins.2.unwrap();
    joins.3.unwrap();
    joins.4.unwrap();
    joins.5.unwrap();
}

fn apply<F, A, B>(fun: F, args: A) -> B
where
    F: Fn(A) -> B,
{
    fun(args)
}

fn _compose<X, Y, Z, F, G>(f: F, g: G) -> impl Fn(X) -> Z
where
    F: Fn(X) -> Y,
    G: Fn(Y) -> Z,
{
    move |x| g(f(x))
}
