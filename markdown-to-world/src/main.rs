use pulldown_cmark::{Event, Parser, TextMergeStream};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), ()> {
    // let dir_path = std::env::args().nth(1).expect("path to directory");

    let dir_path = env::args()
        .nth(1)
        .expect("Please provide a path to the directory");

    let paths = fs::read_dir(&dir_path).expect("Failed to read directory");

    for path in paths {
        let path = path.expect("Failed to get path");
        let file_path = path.path();

        if file_path.is_file() {
            let file_content = fs::read_to_string(&file_path).expect("Failed to read file");

            // Parse the file content here
            // println!("File content: {}", file_content);

            let iterator = TextMergeStream::new(Parser::new(file_content.as_str()));

            let mut max_nesting = 0;
            let mut level = 0;
            for event in iterator {
                match event {
                    Event::Start(tag) => {
                        println!("<{:?}>", tag);
                        level += 1;
                        max_nesting = std::cmp::max(max_nesting, level);
                    }
                    Event::End(tag) => {
                        println!("</{:?}>", tag);
                        level -= 1;
                    }
                    Event::Text(text) => println!("{}", text),
                    _ => {}
                }
            }

            println!("{:?}", max_nesting);
        }
    }

    Ok(())
}
