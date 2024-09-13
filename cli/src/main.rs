use pray_engine::parse;
use pray_engine::Config;
use std::fs;
use tera::Context;
use tera::Tera;

fn main() {
    let path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", path);

    let config_string = fs::read_to_string(path).expect("Unable to read config file");
    let config: Config = parse(&config_string);

    let tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    fs::create_dir_all("generated").expect("failed to create the 'generated' folder");

    let context =
        &Context::from_serialize(&config).expect("Faild to serialise the config into tera context");
    let str = tera
        .render("test.cairo.tera", &context)
        .expect("unable to render template");
    fs::write("generated/test.cairo", str).expect("Unable to write file");
}
