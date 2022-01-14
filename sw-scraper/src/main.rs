extern crate log;
extern crate reqwest;

mod error;
mod model;
use log::info;
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
            info!("factorial - {:?}", next_url_to_fetch);

            let sr = reqwest::blocking::get(next_url_to_fetch)
                .unwrap()
                .json::<Value>()
                .unwrap();

            let next_page = &sr["next"];

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
fn write_to_file<T>(file_name: String, f: impl Fn() -> Collection<T>) -> Result<(), AppError>
where
    T: Serialize,
{
    let mut file = apply(to_path, file_name).map_err(|e| AppError {
        message: Some(String::from("failed to create file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::WriteError,
    })?;
    let content = apply(to_bytes, f()).map_err(|e| AppError {
        message: Some(String::from("failed to create content")),
        cause: None,
        error_type: error::AppErrorType::WriteError,
    })?;

    file.write_all(content.as_bytes()).map_err(|e| AppError {
        message: Some(String::from("failed to write all to file")),
        cause: Some(e.to_string()),
        error_type: error::AppErrorType::WriteError,
    })
}
fn fetch_all_pages(url: Url) -> Vec<Value> {
    let results = vec![];
    let active_url: NextUrlToFetch = Factor::factorial(NextUrlToFetch {
        url: Some(url),
        results,
    });

    active_url.results
}
fn format_url(base: String) -> impl Fn(EntityType) -> Url {
    move |entity_type| -> Url { format!("{}/{}/", &base, entity_type) }
}

fn to_path(file_name: String) -> Result<File, std::io::Error> {
    File::create(Path::new(&file_name))
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
    //init logging
    env_logger::init();

    info!("main - init app config");

    //create app config
    let mut app_config = config::Config::default();

    //load the app_config.toml file
    info!("main - load app config toml file");
    app_config
        .merge(config::File::with_name("app_config"))
        .unwrap();

    let base_url: String = app_config.get("BASE_URL").unwrap();
    let output_dir: String = app_config.get("OUTPUT_DIR").unwrap();

    let build_entity_url = |entity_type: EntityType| -> Url {
        let u = apply(format_url, (&base_url).to_string());
        let url: Url = u(entity_type);
        url
    };

    //create base output dir
    info!("main - creating base output dir");

    fs::create_dir::<_>(&output_dir).unwrap();

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        info!("main - load  films");
        //Film
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::Film))
                .into_iter()
                .collect::<Collection<Film>>()
        };
        info!("main - write films");

        write_to_file(format!("{}/Film.json", &output_dir), find_all);
        info!("main - done  films");
    }));

    handles.push(tokio::spawn(async move {
        info!("main - load  Planet");
        //Planet
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::Planet))
                .into_iter()
                .collect::<Collection<Planet>>()
        };
        info!("main - write Planet");

        write_to_file(format!("{}/Planet.json", &output_dir), find_all);
        info!("main - done  Planet");
    }));

    handles.push(tokio::spawn(async move {
        info!("main - load  Species");
        //Species
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::Species))
                .into_iter()
                .collect::<Collection<Species>>()
        };
        info!("main - write Planet");

        write_to_file(format!("{}/Species.json", &output_dir), find_all);
        info!("main - done  Species");
    }));

    handles.push(tokio::spawn(async move {
        info!("main - load  Vehicle");
        //Vehicle
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::Vehicle))
                .into_iter()
                .collect::<Collection<Vehicle>>()
        };
        info!("main - write Vehicle");

        write_to_file(format!("{}/Vehicle.json", &output_dir), find_all);
        info!("main - done  Vehicle");
    }));

    handles.push(tokio::spawn(async move {
        info!("main - load  Starship");
        //Starship
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::Starship))
                .into_iter()
                .collect::<Collection<Starship>>()
        };
        info!("main - write Starship");

        write_to_file(format!("{}/Starship.json", &output_dir), find_all);
        info!("main - done  Starship");
    }));

    handles.push(tokio::spawn(async move {
        info!("main - load  People");
        //People
        let find_all = || {
            fetch_all_pages(build_entity_url(EntityType::People))
                .into_iter()
                .collect::<Collection<People>>()
        };
        info!("main - write People");

        write_to_file(format!("{}/People.json", &output_dir), find_all);
        info!("main - done  People");
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
