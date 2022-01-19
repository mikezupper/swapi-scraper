extern crate reqwest;
use serde::{Deserialize, Serialize};
use std::{fmt::{self, Debug}, iter::FromIterator};

use serde_json::Value;

//use crate::error::AppError;

pub type Url = String;
//pub type Id = u32;
//pub type EntityFetcher = dyn Fn(Id) -> Result<Value, AppError>;
//pub type FetchEntityFromUrl = dyn Fn(Url) -> FetchEntity;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Film,
    People,
    Planet,
    Species,
    Starship,
    Vehicle,
}
impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntityType::Film => write!(f, "films"),
            EntityType::People => write!(f, "people"),
            EntityType::Planet => write!(f, "planets"),
            EntityType::Species => write!(f, "species"),
            EntityType::Starship => write!(f, "starships"),
            EntityType::Vehicle => write!(f, "vehicles"),
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

#[derive(Serialize, Deserialize, Debug, Default)]
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

impl From<Value> for Starship {
    fn from(v: Value) -> Self {
        let mut p = Starship::default();
        p.name = v["name"].as_str().unwrap().to_string();
        p.model = v["model"].as_str().unwrap().to_string();
        p.manufacturer = v["manufacturer"].as_str().unwrap().to_string();
        p.cost_in_credits = v["cost_in_credits"].as_str().unwrap().to_string();
        p.length = v["length"].as_str().unwrap().to_string();
        p.max_atmosphering_speed = v["max_atmosphering_speed"].as_str().unwrap().to_string();
        p.crew = v["crew"].as_str().unwrap().to_string();
        p.passengers = v["passengers"].as_str().unwrap().to_string();
        p.cargo_capacity = v["cargo_capacity"].as_str().unwrap().to_string();
        p.consumables = v["consumables"].as_str().unwrap().to_string();
        p.hyperdrive_rating = v["hyperdrive_rating"].as_str().unwrap().to_string();
        p.mglt = v["MGLT"].as_str().unwrap().to_string();
        p.starship_class = v["starship_class"].as_str().unwrap().to_string();
        p
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]

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

impl From<Value> for Planet {
    fn from(v: Value) -> Self {
        let mut p = Planet::default();
        p.name = v["name"].as_str().unwrap().to_string();
        p.rotation_period = v["rotation_period"].as_str().unwrap().to_string();
        p.orbital_period = v["orbital_period"].as_str().unwrap().to_string();
        p.diameter = v["diameter"].as_str().unwrap().to_string();
        p.climate = v["climate"].as_str().unwrap().to_string();
        p.gravity = v["gravity"].as_str().unwrap().to_string();
        p.terrain = v["terrain"].as_str().unwrap().to_string();
        p.surface_water = v["surface_water"].as_str().unwrap().to_string();
        p.population = v["population"].as_str().unwrap().to_string();
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

impl From<Value> for People {
    fn from(v: Value) -> Self {
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

#[derive(Serialize, Deserialize, Debug, Clone, Default,PartialEq)]
pub struct Film {
    #[serde(skip_deserializing)]
    id: String,
    title: String,
    episode_id: u64,
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
impl From<Value> for Film {
    fn from(v: Value) -> Self {
        let mut p = Film::default();
        p.title = v["title"].as_str().unwrap().to_string();
        p.episode_id = v["episode_id"].as_u64().unwrap();
        p.opening_crawl = v["opening_crawl"].as_str().unwrap().to_string();
        p.director = v["director"].as_str().unwrap().to_string();
        p.producer = v["producer"].as_str().unwrap().to_string();
        p.release_date = v["release_date"].as_str().unwrap().to_string();
        p
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
impl From<Value> for Species {
    fn from(v: Value) -> Self {
        let mut p = Species::default();
        p.name = v["name"].as_str().unwrap().to_string();
        p.classification = v["classification"].as_str().unwrap().to_string();
        p.designation = v["designation"].as_str().unwrap().to_string();
        p.average_height = v["average_height"].as_str().unwrap().to_string();
        p.average_height = v["average_height"].as_str().unwrap().to_string();
        p.average_lifespan = v["average_lifespan"].as_str().unwrap().to_string();
        p.eye_colors = v["eye_colors"].as_str().unwrap().to_string();
        p.hair_colors = v["hair_colors"].as_str().unwrap().to_string();
        p
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
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
impl From<Value> for Vehicle {
    fn from(v: Value) -> Self {
        let mut p = Vehicle::default();
        p.name = v["name"].as_str().unwrap().to_string();
        p.cargo_capacity = v["cargo_capacity"].as_str().unwrap().to_string();
        p.consumables = v["consumables"].as_str().unwrap().to_string();
        p.cost_in_credits = v["cost_in_credits"].as_str().unwrap().to_string();
        p.crew = v["crew"].as_str().unwrap().to_string();
        p.length = v["length"].as_str().unwrap().to_string();
        p.manufacturer = v["manufacturer"].as_str().unwrap().to_string();
        p.max_atmosphering_speed = v["max_atmosphering_speed"].as_str().unwrap().to_string();
        p.model = v["model"].as_str().unwrap().to_string();
        p.passengers = v["passengers"].as_str().unwrap().to_string();
        p
    }
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Collection<T> {
    pub(crate) results: Vec<T>,
}

impl<Film> Collection<Film> {
    pub(crate) fn new(results: Vec<Film>) -> Self {
        Self { results }
    }

    // pub(crate) fn add(&mut self, elem: Film) {
    //     self.results.push(elem);
    // }
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

impl FromIterator<Value> for Collection<People> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        Collection::new(iter.into_iter().map(|f| People::from(f)).collect())
    }
}
