#[macro_use]
extern crate rocket;

use std::env;

use rocket::fs::FileServer;
use rocket::response::content::RawJson;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::data::{Limits, ToByteUnit};
use rocket::{Request, http::Status};


#[derive(Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
#[serde(tag="kind")]
enum Reply {
    Error {
        message: String,
    }
}


#[derive(Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
struct Parameter {
    name: String,
    integer_bounds: Option<(i64, i64)>,
    real_bounds: Option<(f64, f64)>,
    is_integer: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
#[serde(tag="kind")]
enum Command {
    ListAll,
    ListSpace {
        spaceid: u64,
    },
    AddResult {
        spaceid: u64,
        sampleid: u64,
        result: f64,
    },
    RequestJob {
        spaceid: u64,
    },
    DefineSpace {
        name: String,
        params: Vec<Parameter>,
    }
}

#[post("/", data = "<command>")]
async fn run_query(command: Json<Command>) -> RawJson<String> {
    let rv = match command.0 {
        Command::ListAll => {
            Reply::Error { message: "Foo!".to_string(), }
        },
        _ => {
            todo!();
        }
    };
    let rv = serde_json::to_string(&rv).unwrap();
    RawJson(rv)
}

#[catch(default)]
fn errhandler(status: Status, req: &Request) -> RawJson<String> {
    let reply = Reply::Error { message: format!("{}: {} ({})", status.code, status.reason_lossy(), req.uri()) };
    let serialised_reply = serde_json::to_string(&reply).unwrap();
    RawJson(serialised_reply)
}

#[launch]
fn rocket() -> _ {
    let port: u16 = match env::var("DELBERT_PORT") {
        Ok(port) => port.as_str().parse().unwrap(),
        Err(_) => "8008".parse().unwrap(),
    };

    // Set a limit of 64MiB for JSON.
    let limits = Limits::default()
        .limit("json", 64.mebibytes());
    let config = rocket::Config {
        port,
        address: [127u8, 0, 0, 1].into(),
        limits,
        ..rocket::Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", FileServer::from("site"))
        .mount("/", routes![run_query])
        .register("/", catchers![errhandler])
}