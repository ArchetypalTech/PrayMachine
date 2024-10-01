use pray_engine::{serialize, Room};
use pray_engine::{Config, Level};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd, TextMergeStream};
use std::env;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
enum States {
    None,
    Title,
    Description,
    YAML,
    Object,
    End,
}

#[derive(Debug)]
struct State {
    pub current: States,
    pub room: Room,
}

impl State {
    pub fn new(room_id: u64) -> State {
        State {
            current: States::None,
            room: Room {
                room_id: room_id,
                room_name: "".to_string(),
                room_description: "".to_string(),
                room_type: "".to_string(),
                objects: None,
                object_ids: Vec::new(),
                dir_obj_ids: Vec::new(),
            },
        }
    }

    pub fn after_event(self, event: &Event) -> Self {
        match self.current {
            States::None => self.none(event),
            States::Title => self.title(event),
            States::Description => self.description(event),
            States::YAML => self.yaml(event),
            States::Object => self.object(event),
            States::End => panic!("Already Reached The End"),
        }
    }

    fn none(mut self, event: &Event) -> Self {
        match event {
            Event::Start(tag) => match &tag {
                Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                } => match level {
                    &HeadingLevel::H1 => self.current = States::Title,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        self
    }
    fn title(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                self.room.room_name.push_str(text.to_string().as_str());
            }
            Event::End(tag) => match &tag {
                TagEnd::Heading(level) => match level {
                    &HeadingLevel::H1 => self.current = States::Description,
                    _ => {}
                },

                _ => {}
            },
            _ => {}
        }
        self
    }

    fn description(mut self, event: &Event) -> Self {
        match event {
            Event::Start(tag) => match &tag {
                Tag::Paragraph => {
                    if let Some(n) = self.room.room_description.chars().last() {
                        if n != '\n' {
                            self.room.room_description.push_str("\n");
                        }
                    }
                }

                Tag::CodeBlock(_) => self.current = States::YAML,
                _ => {}
            },
            Event::Text(text) => {
                self.room
                    .room_description
                    .push_str(text.to_string().as_str());
            }
            _ => {}
        }
        self
    }

    fn yaml(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                println!("YAML: {:?}", text.to_string());
                self.current = States::Object;
            }
            _ => {}
        }
        self
    }

    fn object(mut self, event: &Event) -> Self {
        match event {
            Event::Start(tag) => match &tag {
                Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                } => match level {
                    &HeadingLevel::H2 => {
                        println!("NEW OBJECT");
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {} //self.current = States::End,
        }
        self
    }
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
            let mut state = State::new(room_id);

            let iterator = TextMergeStream::new(Parser::new(file_content.as_str()));

            for event in iterator {
                match &event {
                    Event::Start(tag) => {
                        println!("<{:?}>", tag);
                    }
                    Event::End(tag) => {
                        println!("</{:?}>", tag);
                    }
                    Event::Text(text) => {
                        println!("{}", text);
                    }
                    _ => {}
                }
                state = state.after_event(&event);

                println!("STATE: {:?}", state.current);
            }

            rooms.push(state.room);
            println!("====================================");
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
