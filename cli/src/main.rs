use glob::glob;
use notify::{RecursiveMode, Result, Watcher};
use pray_engine::parse;
use pray_engine::Config;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;
use tera::Context;
use tera::Tera;
use tera::Value;

fn get_file_list(parent_dir: &str, pattern: &str) -> Result<Vec<String>> {
    let mut file_list = Vec::new();

    for entry in glob(&pattern).expect("invalid pattern") {
        match entry {
            Ok(path) => {
                if let Ok(relative_path) = path.strip_prefix(parent_dir) {
                    file_list.push(relative_path.to_string_lossy().into_owned());
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(file_list)
}

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

fn write(config_path: &String, destination_path: &String, tera: &Tera, files: &Vec<String>) {
    fs::create_dir_all(destination_path).expect("failed to create the destination folder");

    let config_string = fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = parse(&config_string);

    for file in files {
        let destination_file_name = file
            .strip_suffix(".tera")
            .expect("failted to remove .tera extension to file name");
        let mut destination_file_path = destination_path.to_owned();
        destination_file_path.push_str("/");
        destination_file_path.push_str(destination_file_name); // TODO

        let context = &Context::from_serialize(&config)
            .expect("Faild to serialise the config into tera context");
        let str = tera
            .render(file, &context) // TODO
            .expect("unable to render template");
        fs::write(destination_file_path, str).expect("Unable to write file");
    }
}

fn main() -> Result<()> {
    let config_path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", config_path);

    let template_path = std::env::args().nth(2).expect("path to template folder");
    println!("template_path: {:?}", template_path);

    let destination_path = std::env::args().nth(3).expect("path to destination folder");
    println!("destination_path: {:?}", destination_path);

    let watch = if let Some(arg) = std::env::args().nth(4) {
        arg == "--watch"
    } else {
        false
    };

    let mut template_glob = template_path.clone();
    template_glob.push_str("/**/*.tera");

    let mut tera = match Tera::new(template_glob.as_str()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    tera.register_filter("linebreaks", linebreaks);

    let files =
        get_file_list(&template_path, &template_glob).expect("failed to get files from glob");

    println!("{:?}", &files);

    write(&config_path, &destination_path, &tera, &files);

    if watch {
        let copy_of_config_path = config_path.clone();
        // Automatically select the best implementation for your platform.
        let mut watcher = notify::recommended_watcher(move |res| match res {
            Ok(event) => {
                println!("event: {:?}", event);
                _ = tera.full_reload();
                write(&copy_of_config_path, &destination_path, &tera, &files);
            }
            Err(e) => println!("watch error: {:?}", e),
        })?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new(template_path.as_str()), RecursiveMode::Recursive)?;
        watcher.watch(Path::new(config_path.as_str()), RecursiveMode::NonRecursive)?;

        println!("watching...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}
