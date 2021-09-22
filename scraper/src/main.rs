mod model;

mod error;
extern crate reqwest;

use std::{fs::File, io::Write, path::Path};

use model::EntityFetcher;
use serde_json::Value;

use crate::{
    error::{AppError, AppErrorType},
    model::{EntityType, Id, People, Url},
};

fn main() -> Result<(), AppError> {
    let id_to_url = |id: Id| -> Url {
        println!("id2url - in {}", id);
        let url: Url = format!("https://api.starwars.run/api/people/{}", id);
        println!("id2url - out {}", url);
        url
    };
    let entity_to_url = |entity: EntityType| -> Url {
        match entity {
            EntityType::People => "https://api.starwars.run/api/people/".to_string(),
            _ => "https://api.starwars.run/api/".to_string(),
        }
    };
    let url_to_person = |url: Url| -> People {
        println!("url2person in {}", url);

        let value = reqwest::blocking::get(url)
            .map_err(|error| AppError {
                message: None,
                cause: Some(error.to_string()),
                error_type: AppErrorType::FetchError,
            })
            .unwrap()
            .json::<Value>()
            .map_err(|_| AppError {
                message: Some(format!("could not fetch entity ")),
                cause: None,
                error_type: AppErrorType::NotFound,
            })
            .unwrap();
        let p = People::from(&value);
        println!("url2person - out  {:?}", p);
        p
    };

    let fetch_person = compose(id_to_url, url_to_person);
    dbg!(fetch_person(1));
    //let fetch_people = compose(entity_to_url, url_to_person);

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
