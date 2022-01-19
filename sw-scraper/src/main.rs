extern crate actix;
extern crate reqwest;

mod error;
mod logger;
mod model;
mod scraper;
mod utils;

use crate::error::AppError;
use crate::logger::ThreadLocalDrain;
use crate::scraper::{FetchPageCommand, UrlFetcher};

use actix::prelude::*;

use serde_json::from_str;
use slog::{debug, info, o};
use slog::{Drain, Logger};
use slog_async;
use slog_term;
use std::collections::HashMap;

#[actix::main]
async fn main() {
    let log = setup_logging();
    // let _scraper_logger = log.new(o!("thread_name"=>"scraper"));
    // let _writer_logger = log.new(o!("thread_name"=>"writer"));

    //create app config
    let mut app_config = config::Config::default();

    //load the app_config.toml file
    app_config
        .merge(config::File::with_name("app_config"))
        .unwrap();
    let base_url: String = app_config
        .get("BASE_URL")
        .map_err(|err| AppError {
            message: Some("failed to load config files".to_string()),
            cause: Some(err.to_string()),
            error_type: error::AppErrorType::ConfigError,
        })
        .unwrap();
    debug!(log, " BASE_URL {}", &base_url);

    let output_dir: String = app_config.get("OUTPUT_DIR").unwrap();
    debug!(log, " OUTPUT_DIR {}", &output_dir);

    let num_threads: usize = app_config.get("NUM_THREADS").unwrap();
    debug!(log, " NUM_THREADS {}", &num_threads);

    //fs::create_dir(output_dir).unwrap();
    info!(log, "fetching base entities");

    let l = log.new(o!("thread_name"=>"url_fetcher"));
    let fetch_url_addr = SyncArbiter::start(num_threads, move || UrlFetcher { logger: l.clone() });
    let resp = fetch_url_addr
        .send(FetchPageCommand {
            entity_type: String::from("root"),
            base_url,
        })
        .await
        .unwrap()
        .unwrap();
    let root_entities: HashMap<String, String> = from_str(&resp).unwrap();

    for n in root_entities {
        let _ = fetch_url_addr
            .send(FetchPageCommand {
                entity_type: n.0.to_string(),
                base_url: n.1.to_string(),
            })
            .await
            .map_err(|err| AppError {
                message: Some("failed to load url".to_string()),
                cause: Some(err.to_string()),
                error_type: error::AppErrorType::_FetchError,
            })
            .unwrap()
            .unwrap();
        // info!(log, "Response came in {:?}", y);
    }
}

fn setup_logging() -> Logger {
    //--- set up slog

    // set up terminal logging
    let decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(decorator).build().fuse();

    // json log file
    let logfile = std::fs::File::create("/var/tmp/actix-test.log").unwrap();
    let json_drain = slog_json::Json::new(logfile)
        .add_default_keys()
        // include source code location
        .add_key_value(o!("place" =>
           slog::FnValue(move |info| {
               format!("{}::({}:{})",
                       info.module(),
                       info.file(),
                       info.line(),
                )}),
                "sha"=> env!("VERGEN_GIT_SHA")))
        .build()
        .fuse();

    // duplicate log to both terminal and json file
    let dup_drain = slog::Duplicate::new(json_drain, term_drain);
    // make it async
    let async_drain = slog_async::Async::new(dup_drain.fuse()).build();
    // and add thread local logging
    let log = slog::Logger::root(ThreadLocalDrain { drain: async_drain }.fuse(), o!());
    log
}

//  fn main2() {
//     //init logging
//     env_logger::init();

//     info!("main - init app config");
//     //create app config
//     let mut app_config = config::Config::default();

//     //load the app_config.toml file
//     info!("main - load app config toml file");
//     app_config
//         .merge(config::File::with_name("app_config"))
//         .unwrap();

//     let base_url: String = app_config
//         .get("BASE_URL")
//         .map_err(|err| AppError {
//             message: Some("failed to load config files".to_string()),
//             cause: Some(err.to_string()),
//             error_type: error::AppErrorType::ConfigError,
//         })
//         .unwrap();

//     let output_dir: String = app_config.get("OUTPUT_DIR").unwrap();

//     info!("main - creating base output dir");
//     fs::create_dir(output_dir).unwrap();

//     debug!("main - base_url  {:?}", &base_url);

//     let base_entities = reqwest::blocking::get(&base_url)
//         .unwrap()
//         .json::<Value>()
//         .unwrap();
//     dbg!(base_entities);
// }
