use actix::prelude::*;
use slog::info;

use crate::error::AppError;

pub(crate) struct _AppState {
    logger: slog::Logger,
}
/// Define message
#[derive(Message, Debug, Default, PartialEq)]
#[rtype(result = "Result<String, reqwest::Error>")]
pub(crate) struct FetchPageCommand {
    pub base_url: String,
    pub entity_type: String,
}

#[derive(Debug)]
pub(crate) struct UrlFetcher {
    pub(crate) logger: slog::Logger,
}

// Provide Actor implementation for our actor
impl Actor for UrlFetcher {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut SyncContext<Self>) {
        info!(self.logger, "UrlFetcher started");
    }

    fn stopped(&mut self, ctx: &mut SyncContext<Self>) {
        info!(self.logger,"UrlFetcher stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<FetchPageCommand> for UrlFetcher {
    type Result = Result<String, reqwest::Error>;

    fn handle(&mut self, msg: FetchPageCommand, ctx: &mut SyncContext<Self>) -> Self::Result {
        let text =reqwest::blocking::get(msg.base_url).unwrap().text();
        if let Ok(_) = text {
            info!(self.logger,"got the text back");
            text
        } else {
            info!(self.logger,"no text???");
            todo!();
        }
        
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SearchResultResponse {
    pub next: Option<String>,
    pub result: Vec<serde_json::Value>,
}

// Actor definition
struct SwScraper {
    name: String,
    recipient: Recipient<FetchPageCommand>,
}

impl Actor for SwScraper {
    type Context = Context<SwScraper>;
}

// simple message handler for Ping message
impl Handler<SearchResultResponse> for SwScraper {
    type Result = ();

    fn handle(&mut self, msg: SearchResultResponse, ctx: &mut Context<Self>) {
        let url = match msg.next {
            Some(it) => it,
            _ => return,
        };
        self.recipient.do_send(FetchPageCommand {
            base_url: url,
            entity_type: "todo!()".to_string(),
        });
    }
}
