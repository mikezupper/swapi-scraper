mod model;

mod error;
extern crate reqwest;

use model::EntityFetcher;
use serde_json::Value;

use crate::{
    error::{AppError, AppErrorType},
    model::{Id, People},
};

fn main() -> Result<(), AppError> {



    
    let anakin = get_person(1);
    dbg!(anakin);

    let boba_fett = get_entity(2)?;
    dbg!(boba_fett);

    let _ = get_entity(100)?;

    // let app_data = AppData::default();

    // let people = RefCell::new(app_data.people);

    // let people_fetcher = |url: String| {
    //     let f = |fetch_url| {
    //         let add_people_result = |results: &mut Vec<People>| people.borrow_mut().append(results);

    //         let mut active_url = Some(String::from(fetch_url));
    //         while let Some(next_url) = active_url {
    //             println!("fetching url: {:?}", next_url);

    //             let mut sr: SearchResult<People> = reqwest::blocking::get(next_url.clone())
    //                 .unwrap()
    //                 .json()
    //                 .unwrap();
    //             println!("found results next url: {:?} {:?}", sr.next, sr.count);

    //             add_people_result(&mut sr.results);
    //             active_url = sr.next;
    //         }
    //     };
    //     println!("returning fetch func for url: {:?}", url);

    //     fetch(f, url)
    // };
    // people_fetcher(String::from("https://api.starwars.run/api/people/"));
    // println!("total people found: {:?}", &people.borrow().len());

    // let path = Path::new("people.json");
    // let display = path.display();

    // // Open a file in write-only mode, returns `io::Result<File>`
    // let mut file = File::create(&path).unwrap();

    // match file.write_all(serde_json::to_string(&people).unwrap().as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
    Ok(())
}

fn _apply<F, A, B>(fun: F, args: A) -> B
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

fn fetch_entity(url: String) -> Box<EntityFetcher> {
    println!("url {:?}", url);

    Box::new(move |id: Id| -> Result<Value, AppError> {
        reqwest::blocking::get(format!("{}{}", url, id))
            .map_err(|error| AppError {
                message: None,
                cause: Some(error.to_string()),
                error_type: AppErrorType::FetchError,
            })?
            .json::<Value>()
            .map_err(|_| AppError {
                message: Some(format!("could not fetch entity by id {}", &id)),
                cause: None,
                error_type: AppErrorType::NotFound,
            })
    })
}

fn get_entity(id: Id) -> Result<Value, AppError> {
    println!("id {:?}", id);
    let fetcher = fetch_entity(format!("https://api.starwars.run/api/people/{}", id));
    fetcher(id)
}

fn get_person(id: Id) -> People {
    println!("id {:?}", id);
    let fetcher = fetch_entity(format!("https://api.starwars.run/api/people/{}", id));
    let person = People::from(fetcher(id).unwrap());
    person
}