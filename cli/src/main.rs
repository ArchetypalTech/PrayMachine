use serde::{Deserialize, Serialize};
use std::fs;
use tera::Context;
use tera::Tera;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Exit {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Object {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Room {
    room_ID: i64,
    room_name: String,
    room_description: String,
    exits: Option<Vec<Exit>>,
    objects: Option<Vec<Object>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Level {
    level_name: String,
    rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    levels: Vec<Level>,
}

fn main() {
    let path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", path);

    let f = std::fs::File::open(path).expect("failed to read config file");
    let config: Config = serde_yml::from_reader(&f).expect("failed to parse yaml config");

    let tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context =
        &Context::from_serialize(&config).expect("Faild to serialise the config into tera context");
    let str = tera
        .render("test.cairo.tera", &context)
        .expect("unable to render template");
    fs::write("cairo/test.cairo", str).expect("Unable to write file");
}
