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
    let config_path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", config_path);

    let template_path = std::env::args().nth(2).expect("path to template folder");
    println!("template_path: {:?}", template_path);

    let destination_path = std::env::args().nth(3).expect("path to destination folder");
    println!("destination_path: {:?}", destination_path);

    let config_string = fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = parse(&config_string);

    let mut template_glob = template_path.to_owned();
    template_glob.push_str("/**/*.tera");

    let mut tera = match Tera::new(template_glob.as_str()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    tera.register_filter("linebreaks", linebreaks);

    fs::create_dir_all(destination_path).expect("failed to create the destination folder");

    let context =
        &Context::from_serialize(&config).expect("Faild to serialise the config into tera context");
    let str = tera
        .render("test.cairo.tera", &context)
        .expect("unable to render template");
    fs::write("generated/test.cairo", str).expect("Unable to write file");
}
