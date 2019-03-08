extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use std::fs;
use hyper::{Body, Response, StatusCode};
use gotham::helpers::http::response::create_response;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    code: String,
}

fn get_zip_handler(state: State) -> (State, Response<Body>) {
    let zip = PathExtractor::borrow_from(&state);
    let sub_zip = &zip.code[0..3];

    let path = format!("./code/{}/{}.json", sub_zip, zip.code);
    let content = fs::read_to_string(path).expect("fail to read file");
    let res = create_response(
        &state, StatusCode::OK, mime::APPLICATION_JSON, content.into_bytes(),
    );
    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/zip/:code")
            .with_path_extractor::<PathExtractor>()
            .to(get_zip_handler);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
