extern crate reqwest;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};

use serde_json::Value;

use crate::error::AppError;

pub type Url = String;
pub type Id = u32;
pub type EntityFetcher = dyn Fn(Id) -> Result<Value, AppError>;
//pub type FetchEntityFromUrl = dyn Fn(Url) -> FetchEntity;

#[derive(Debug, Default)]
pub struct AppData {
    pub people: Vec<People>,
    _starships: Vec<Starship>,
    _vehicles: Vec<Vehicle>,
    _planets: Vec<Planet>,
    _species: Vec<Species>,
    _films: Vec<Film>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootEntity {
    name: String,
    url: Url,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Film,
    People,
    Planet,
    SearchResult,
    Species,
    Starship,
    Vehicle,
}
impl FromStr for EntityType {
    type Err = ();
    fn from_str(input: &str) -> Result<EntityType, Self::Err> {
        match input {
            "people" => Ok(EntityType::People),
            "planets" => Ok(EntityType::Planet),
            "species" => Ok(EntityType::Species),
            "films" => Ok(EntityType::Film),
            "starships" => Ok(EntityType::Starship),
            "vehicles" => Ok(EntityType::Vehicle),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

impl<T> From<Response> for SearchResult<T> {
    fn from(_: Response) -> Self {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Starship {
    name: String,
    model: String,
    manufacturer: String,
    cost_in_credits: String,
    length: String,
    max_atmosphering_speed: String,
    crew: String,
    passengers: String,
    cargo_capacity: String,
    consumables: String,
    hyperdrive_rating: String,
    #[serde(rename(deserialize = "MGLT"))]
    mglt: String,
    starship_class: String,
    pilots: Vec<Url>,
    films: Vec<Url>,
    url: Url,
    created: String,
    edited: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Planet {
    name: String,
    rotation_period: String,
    orbital_period: String,
    diameter: String,
    climate: String,
    gravity: String,
    terrain: String,
    surface_water: String,
    population: String,
    residents: Vec<Url>,
    films: Vec<Url>,
    url: Url,
    created: String,
    edited: String,
}

impl From<&Value> for People {
    fn from(v: &Value) -> Self {
        let mut p = People::default();
        p.name = v["name"].as_str().unwrap().to_string();
        p.height = v["height"].as_str().unwrap().to_string();
        p.hair_color = v["hair_color"].as_str().unwrap().to_string();
        p.mass = v["mass"].as_str().unwrap().to_string();
        p.skin_color = v["skin_color"].as_str().unwrap().to_string();
        p.eye_color = v["eye_color"].as_str().unwrap().to_string();
        p.birth_year = v["birth_year"].as_str().unwrap().to_string();
        p.gender = v["gender"].as_str().unwrap().to_string();
        p.homeworld = v["homeworld"].as_str().unwrap().to_string();
        p
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct People {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub hair_color: String,
    pub skin_color: String,
    pub eye_color: String,
    pub birth_year: String,
    pub gender: String,
    pub homeworld: String,
    pub films: Vec<Url>,
    pub species: Vec<Url>,
    pub vehicles: Vec<Url>,
    pub starships: Vec<Url>,
    pub url: Url,
    pub created: String,
    pub edited: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Film {
    #[serde(skip_deserializing)]
    id: String,
    title: String,
    episode_id: i32,
    opening_crawl: String,
    director: String,
    producer: String,
    release_date: String,
    species: Vec<Url>,
    vehicles: Vec<Url>,
    starships: Vec<Url>,
    characters: Vec<Url>,
    planets: Vec<Url>,
    url: Url,
    created: String,
    edited: String,
}
// impl Film {
//     fn new(film:Film)->Self{
//         Self{
//             id:Uuid::new_v4().to_string(),
//         ..film
//         }
//     }
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct Species {
    name: String,
    classification: String,
    designation: String,
    average_height: String,
    average_lifespan: String,
    eye_colors: String,
    hair_colors: String,
    skin_colors: String,
    language: String,
    homeworld: Option<Url>,
    people: Vec<Url>,
    films: Vec<Url>,
    url: Url,
    created: String,
    edited: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    cargo_capacity: String,
    consumables: String,
    cost_in_credits: String,
    crew: String,
    length: String,
    manufacturer: String,
    max_atmosphering_speed: String,
    model: String,
    name: String,
    passengers: String,
    films: Vec<Url>,
    pilots: Vec<Url>,
    vehicle_class: String,
    url: Url,
    created: String,
    edited: String,
}
