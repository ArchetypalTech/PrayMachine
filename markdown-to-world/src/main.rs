use pray_engine::{serialize, Object, Room};
use pray_engine::{Config, Level};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd, TextMergeStream};
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::{env, vec};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomYaml {
    pub room_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectYaml {
    pub direction: Option<String>,
    #[serde(rename = "type")]
    pub ttype: String,
    pub material: String,
}

#[derive(Debug, PartialEq, Eq)]
enum RoomStateMachineStates {
    None,
    RoomName,
    RoomDescription,
    RoomYAML,
    Object,
    End,
}

#[derive(Debug)]
struct RoomStateMachine {
    pub state: RoomStateMachineStates,
    pub room: Room,
    pub current_object: Option<ObjectStateMachine>,
}

impl RoomStateMachine {
    pub fn new(room_id: u64) -> RoomStateMachine {
        RoomStateMachine {
            state: RoomStateMachineStates::None,
            room: Room {
                room_id: room_id,
                room_name: "".to_string(),
                room_description: "".to_string(),
                room_type: "".to_string(),
                objects: Some(vec![]),
                object_ids: Vec::new(),
                dir_obj_ids: Vec::new(),
            },
            current_object: None,
        }
    }

    pub fn after_event(self, event: &Event) -> Self {
        match self.state {
            RoomStateMachineStates::None => self.none(event),
            RoomStateMachineStates::RoomName => self.room_name(event),
            RoomStateMachineStates::RoomDescription => self.room_description(event),
            RoomStateMachineStates::RoomYAML => self.room_yaml(event),
            RoomStateMachineStates::Object => self.object(event),
            RoomStateMachineStates::End => panic!("Already Reached The End"),
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
                    &HeadingLevel::H1 => self.state = RoomStateMachineStates::RoomName,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        self
    }
    fn room_name(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                self.room.room_name.push_str(text.to_string().as_str());
            }
            Event::End(tag) => match &tag {
                TagEnd::Heading(level) => match level {
                    &HeadingLevel::H1 => self.state = RoomStateMachineStates::RoomDescription,
                    _ => {}
                },

                _ => {}
            },
            _ => {}
        }
        self
    }

    fn room_description(mut self, event: &Event) -> Self {
        match event {
            Event::Start(tag) => match &tag {
                Tag::Paragraph => {
                    if let Some(n) = self.room.room_description.chars().last() {
                        if n != '\n' {
                            self.room.room_description.push_str("\n");
                        }
                    }
                }

                Tag::CodeBlock(_) => self.state = RoomStateMachineStates::RoomYAML,
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

    fn room_yaml(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                let room_yaml: RoomYaml =
                    serde_yml::from_str(&text.to_string()).expect("failed to parse yaml config");
                self.room.room_type = room_yaml.room_type;
                self.state = RoomStateMachineStates::Object
            }
            _ => {}
        }
        self
    }

    fn object(mut self, event: &Event) -> Self {
        if let Some(object_state_machine) = self.current_object {
            let new_state = object_state_machine.after_event(&event);
            if new_state.state == ObjectStateMachineStates::End {
                let obj = new_state.object;
                self.current_object = None;
                if let Some(ref mut vector) = self.room.objects {
                    vector.push(obj);
                }
            } else {
                self.current_object = Some(new_state);
            }
        } else {
            match event {
                Event::Start(tag) => match &tag {
                    Tag::Heading {
                        level,
                        id: _,
                        classes: _,
                        attrs: _,
                    } => match level {
                        &HeadingLevel::H2 => {
                            // TODO object_id
                            self.current_object = Some(ObjectStateMachine::new(0))
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {} //self.current = States::End,
            }
        }

        self
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ObjectStateMachineStates {
    ObjectTitle,
    ObjectDescription,
    ObjectYAML,
    End,
}

#[derive(Debug)]
struct ObjectStateMachine {
    pub state: ObjectStateMachineStates,
    pub object: Object,
}

impl ObjectStateMachine {
    pub fn new(object_id: u64) -> ObjectStateMachine {
        ObjectStateMachine {
            state: ObjectStateMachineStates::ObjectDescription,
            object: Object {
                obj_id: object_id,
                actions: None,
                destination: None,
                direction: None,
                material: "".to_string(),
                obj_description: "".to_string(),
                ttype: "".to_string(),
            },
        }
    }

    pub fn after_event(self, event: &Event) -> Self {
        match self.state {
            ObjectStateMachineStates::ObjectTitle => self.title(event),
            ObjectStateMachineStates::ObjectDescription => self.description(event),
            ObjectStateMachineStates::ObjectYAML => self.yaml(event),
            ObjectStateMachineStates::End => panic!("Already Reached The End"),
        }
    }

    fn title(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                self.object
                    .obj_description
                    .push_str(text.to_string().as_str());
            }
            Event::End(tag) => match &tag {
                TagEnd::Heading(level) => match level {
                    &HeadingLevel::H2 => self.state = ObjectStateMachineStates::ObjectDescription,
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
                    if let Some(n) = self.object.obj_description.chars().last() {
                        if n != '\n' {
                            self.object.obj_description.push_str("\n");
                        }
                    }
                }

                Tag::CodeBlock(kind) => self.state = ObjectStateMachineStates::ObjectYAML,

                _ => {}
            },
            Event::Text(text) => {
                self.object
                    .obj_description
                    .push_str(text.to_string().as_str());
            }
            _ => {}
        }
        self
    }

    fn yaml(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                let object_yaml: ObjectYaml =
                    serde_yml::from_str(&text.to_string()).expect("failed to parse yaml config");
                self.object.material = object_yaml.material;
                self.object.ttype = object_yaml.ttype;
                if let Some(direction) = object_yaml.direction {
                    self.object.direction = Some(direction)
                }
                self.state = ObjectStateMachineStates::End
            }
            _ => {}
        }
        self
    }
}

fn main() -> Result<(), ()> {
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
            let mut state_machine = RoomStateMachine::new(room_id);

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
                state_machine = state_machine.after_event(&event);

                println!("STATE: {:?}", state_machine.state);
            }

            rooms.push(state_machine.room);
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
