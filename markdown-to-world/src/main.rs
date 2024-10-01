use pray_engine::{serialize, Room};
use pray_engine::{Config, Level};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd, TextMergeStream};
// use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

enum State {
    None,
    Title,
    Description,
    Next,
}

fn main() -> Result<(), ()> {
    // let rooms: HashMap<String, Room> = HashMap::new();
    let mut rooms: Vec<Room> = Vec::new();

    let dir_path = env::args()
        .nth(1)
        .expect("Please provide a path to the directory");

    let config_path = env::args()
        .nth(2)
        .expect("Please provide a path to the config file to generate");

    let paths = fs::read_dir(&dir_path).expect("Failed to read directory");

    for path in paths {
        let path = path.expect("Failed to get path");
        let file_path = path.path();

        if file_path.is_file() {
            let file_content = fs::read_to_string(&file_path).expect("Failed to read file");

            let room_id = calculate_hash(&file_path.to_str());

            let mut room = Room {
                room_id: room_id,
                room_name: "".to_string(),
                room_description: "".to_string(),
                room_type: "".to_string(),
                objects: None,
                object_ids: Vec::new(),
                dir_obj_ids: Vec::new(),
            };

            // Parse the file content here
            // println!("File content: {}", file_content);

            let iterator = TextMergeStream::new(Parser::new(file_content.as_str()));

            let mut max_nesting = 0;
            let mut nesting_level = 0;

            let mut state = State::None;

            for event in iterator {
                match event {
                    Event::Start(tag) => {
                        match &tag {
                            Tag::Heading {
                                level,
                                id,
                                classes,
                                attrs,
                            } => match level {
                                &HeadingLevel::H1 => {
                                    state = State::Title;
                                }
                                // &HeadingLevel::H2 => {
                                //     state = State::Next;
                                // }
                                _ => {}
                            },
                            Tag::CodeBlock(d) => {
                                state = State::Next;
                            }
                            Tag::Paragraph => match state {
                                State::Description => {
                                    if let Some(n) = room.room_description.chars().last() {
                                        if n != '\n' {
                                            room.room_description.push_str("\n");
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                        println!("<{:?}>", tag);
                        nesting_level += 1;
                        max_nesting = std::cmp::max(max_nesting, nesting_level);
                    }
                    Event::End(tag) => {
                        println!("</{:?}>", tag);
                        nesting_level -= 1;
                        match &tag {
                            TagEnd::Heading(level) => match level {
                                &HeadingLevel::H1 => {
                                    state = State::Description;
                                }
                                &HeadingLevel::H2 => {
                                    // state = State::Description;
                                }
                                _ => {}
                            },
                            TagEnd::Paragraph => {}
                            _ => {}
                        }
                    }
                    Event::Text(text) => {
                        match state {
                            State::Title => {
                                room.room_name.push_str(text.to_string().as_str());
                            }
                            State::Description => {
                                room.room_description.push_str(text.to_string().as_str());
                            }
                            _ => {}
                        }

                        println!("{}", text);
                    }
                    _ => {}
                }
            }

            rooms.push(room);

            println!("{:?}", max_nesting);
        }
    }

    let config_str = serialize(&Config {
        levels: vec![Level {
            level_name: "test".to_string(),
            rooms: rooms,
        }],
    });

    fs::write(config_path, config_str).expect("Unable to write file");

    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
