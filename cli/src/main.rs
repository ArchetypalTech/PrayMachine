use pray_engine::parse;
use pray_engine::Config;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use tera::Context;
use tera::Tera;
use tera::Value;

/// Convert line breaks
pub fn linebreaks(value: &Value, params: &HashMap<String, Value>) -> tera::Result<Value> {
    let text: String = if let Value::String(s) = value {
        s.to_string()
    } else {
        return Err("The linebreaks filter can only be applied to strings".into());
    };
    let mut replacement_text: String = String::from_str("\\n").expect("failed to convert");
    let default_value = Value::String(replacement_text);
    let replacement = params.get("to");
    let replacement_value = if let Option::Some(v) = replacement {
        v
    } else {
        &default_value
    };

    if let Value::String(s) = replacement_value {
        replacement_text = s.to_owned()
    } else {
        return Err("The linebreaks filter can only be applied to strings".into());
    };
    Ok(Value::String(
        text.replace("\r\n", replacement_text.as_str())
            .replace("\n", replacement_text.as_str()),
    ))
}

fn main() {
    let path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", path);

    let config_string = fs::read_to_string(path).expect("Unable to read config file");
    let config: Config = parse(&config_string);

    let mut tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    tera.register_filter("linebreaks", linebreaks);

    fs::create_dir_all("generated").expect("failed to create the 'generated' folder");

    let context =
        &Context::from_serialize(&config).expect("Faild to serialise the config into tera context");
    let str = tera
        .render("test.cairo.tera", &context)
        .expect("unable to render template");
    fs::write("generated/test.cairo", str).expect("Unable to write file");
}
