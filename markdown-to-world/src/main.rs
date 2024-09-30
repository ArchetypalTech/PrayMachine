use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd, TextMergeStream};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Room {
    id: String,
    name: String,
    description: String,
}

struct PartialRoom {
    id: String,
    name: String,
    description: String,
}

impl From<PartialRoom> for Room {
    fn from(base: PartialRoom) -> Self {
        Room {
            id: base.id,
            name: base.name,
            description: base.description,
        }
    }
}

enum State {
    None,
    Title,
    Description,
    Next,
}

fn main() -> Result<(), ()> {
    let rooms: HashMap<String, Room> = HashMap::new();

    let dir_path = env::args()
        .nth(1)
        .expect("Please provide a path to the directory");

    let paths = fs::read_dir(&dir_path).expect("Failed to read directory");

    for path in paths {
        let path = path.expect("Failed to get path");
        let file_path = path.path();

        if file_path.is_file() {
            let file_content = fs::read_to_string(&file_path).expect("Failed to read file");

            let mut room = PartialRoom {
                id: file_path.to_str().unwrap().to_string(),
                name: "".to_string(),
                description: "".to_string(),
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
                                    if let Some(n) = room.description.chars().last() {
                                        if n != '\n' {
                                            room.description.push_str("\n");
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
                                room.name.push_str(text.to_string().as_str());
                            }
                            State::Description => {
                                room.description.push_str(text.to_string().as_str());
                            }
                            _ => {}
                        }

                        println!("{}", text);
                    }
                    _ => {}
                }
            }

            let room: Room = room.into();
            println!("============================================");
            println!("{:?}", room);
            println!("============================================");

            println!("{:?}", max_nesting);
        }
    }

    Ok(())
}
