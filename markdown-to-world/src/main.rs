use pray_engine::{serialize, Action, Effect, Object, Room};
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

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct EffectYaml {
//     #[serde(rename = "roomID")]
//     pub room_id: Option<u64>,
//     #[serde(rename = "objectID")]
//     pub object_id: Option<u64>,
//     #[serde(rename = "actionID")]
//     pub action_id: u64,
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionYaml {
    #[serde(rename = "type")]
    pub ttype: String, // TODO enum?
    pub enabled: Option<bool>,
    pub revertable: Option<bool>,
    pub d_bit: Option<bool>,
    pub affects_action: Option<Effect>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RoomStateMachineStates {
    None,
    RoomName,
    RoomDescription,
    RoomYAML,
    Object,
    // End,
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
        let previous_state = self.state.clone();
        let s = match self.state {
            RoomStateMachineStates::None => self.none(event),
            RoomStateMachineStates::RoomName => self.room_name(event),
            RoomStateMachineStates::RoomDescription => self.room_description(event),
            RoomStateMachineStates::RoomYAML => self.room_yaml(event),
            RoomStateMachineStates::Object => self.object(event),
            // RoomStateMachineStates::End => panic!("Already Reached The End"),
        };

        if previous_state != s.state {
            println!("= = = =");
            println!("ROOM {:?}", s.state);
            println!("= = = =");
        }
        s
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
        let mut match_event = true;
        if let Some(object_state_machine) = self.current_object {
            match_event = false;
            let new_state = object_state_machine.after_event(&event);
            if new_state.state == ObjectStateMachineStates::End {
                let obj = new_state.object;
                self.current_object = None;
                if let Some(ref mut vector) = self.room.objects {
                    vector.push(obj);
                }
                match_event = true;
            } else {
                self.current_object = Some(new_state);
            }
        }

        if match_event {
            match event {
                Event::Start(tag) => match &tag {
                    Tag::Heading {
                        level,
                        id: _,
                        classes: _,
                        attrs: _,
                    } => match level {
                        &HeadingLevel::H2 => {
                            let object_id = calculate_object_id(
                                self.room.room_id.clone(),
                                self.room.objects.clone().unwrap().len().try_into().unwrap(),
                            );
                            let sm = ObjectStateMachine::new(object_id);
                            let state = sm.state.clone();
                            self.current_object = Some(sm);
                            println!("= = = =");
                            println!("OBJECT {:?}", state);
                            println!("= = = =");
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ObjectStateMachineStates {
    ObjectDescription,
    ObjectYAML,
    ObjectActions,
    End,
}

#[derive(Debug)]
struct ObjectStateMachine {
    pub state: ObjectStateMachineStates,
    pub object: Object,
    pub current_action: Option<ActionStateMachine>,
}

impl ObjectStateMachine {
    pub fn new(object_id: u64) -> ObjectStateMachine {
        ObjectStateMachine {
            state: ObjectStateMachineStates::ObjectDescription,
            object: Object {
                obj_id: object_id,
                actions: Some(vec![]),
                destination: None,
                direction: None,
                material: "".to_string(),
                obj_description: "".to_string(),
                ttype: "".to_string(),
            },
            current_action: None,
        }
    }

    pub fn after_event(self, event: &Event) -> Self {
        let previous_state = self.state.clone();
        let s = match self.state {
            ObjectStateMachineStates::ObjectDescription => self.description(event),
            ObjectStateMachineStates::ObjectYAML => self.yaml(event),
            ObjectStateMachineStates::ObjectActions => self.actions(event),
            ObjectStateMachineStates::End => panic!("Already Reached The End"),
        };

        if previous_state != s.state {
            println!("= = = =");
            println!("OBJECT {:?}", s.state);
            println!("= = = =");
        }
        s
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

                Tag::CodeBlock(_kind) => self.state = ObjectStateMachineStates::ObjectYAML,

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
                self.state = ObjectStateMachineStates::ObjectActions
            }
            _ => {}
        }
        self
    }

    fn actions(mut self, event: &Event) -> Self {
        let mut match_event = true;
        if let Some(action_state_machine) = self.current_action {
            match_event = false;
            let new_state = action_state_machine.after_event(&event);
            if new_state.state == ActionStateMachineStates::End {
                let action = new_state.action;
                self.current_action = None;
                if let Some(ref mut vector) = self.object.actions {
                    vector.push(action);
                }
                match_event = true;
                if let Some(destination) = new_state.destination {
                    if let Some(_current_destination) = self.object.destination {
                        panic!("only one destination per object for now");
                    } else {
                        self.object.destination = Some(destination);
                    }
                }
            } else {
                self.current_action = Some(new_state);
            }
        }
        if match_event {
            match event {
                Event::Start(tag) => match &tag {
                    Tag::Heading {
                        level,
                        id: _,
                        classes: _,
                        attrs: _,
                    } => match level {
                        &HeadingLevel::H4 => {
                            let action_id = calculate_action_id(
                                self.object.obj_id.clone(),
                                self.object
                                    .actions
                                    .clone()
                                    .unwrap()
                                    .len()
                                    .try_into()
                                    .unwrap(),
                            );
                            let sm = ActionStateMachine::new(action_id);
                            let state = sm.state.clone();
                            self.current_action = Some(sm);
                            println!("= = = =");
                            println!("ACTION {:?}", state);
                            println!("= = = =");
                        }
                        &HeadingLevel::H2 => self.state = ObjectStateMachineStates::End,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ActionStateMachineStates {
    ActionEffectDescription,
    ActionYAML,
    End,
}

#[derive(Debug)]
struct ActionStateMachine {
    pub state: ActionStateMachineStates,
    pub action: Action,
    pub destination: Option<String>,
}

impl ActionStateMachine {
    pub fn new(action_id: u64) -> ActionStateMachine {
        ActionStateMachine {
            state: ActionStateMachineStates::ActionEffectDescription,
            action: Action {
                action_id: action_id,
                affects_action: None,
                d_bit: true,
                d_bit_text: "".to_string(),
                enabled: true,
                revertable: false,
                ttype: "".to_string(),
            },
            destination: None,
        }
    }

    pub fn after_event(self, event: &Event) -> Self {
        let previous_state = self.state.clone();
        let s = match self.state {
            ActionStateMachineStates::ActionEffectDescription => self.effect_description(event),
            ActionStateMachineStates::ActionYAML => self.yaml(event),
            ActionStateMachineStates::End => panic!("Already Reached The End"),
        };

        if previous_state != s.state {
            println!("= = = =");
            println!("ACTION: {:?}", s.state);
            println!("= = = =");
        }
        s
    }

    fn effect_description(mut self, event: &Event) -> Self {
        match event {
            Event::Start(tag) => match &tag {
                Tag::Paragraph => {
                    if let Some(n) = self.action.d_bit_text.chars().last() {
                        if n != '\n' {
                            self.action.d_bit_text.push_str("\n");
                        }
                    }
                }

                Tag::Link {
                    link_type: _,
                    dest_url,
                    title,
                    id: _,
                } => {
                    self.action.d_bit_text = title.to_string();
                    self.destination = Some(dest_url.to_string());
                    self.action.ttype = "open".to_string();
                }

                Tag::CodeBlock(_kind) => self.state = ActionStateMachineStates::ActionYAML,

                _ => {}
            },

            Event::End(tag) => match &tag {
                TagEnd::Link => self.state = ActionStateMachineStates::End,
                TagEnd::CodeBlock => self.state = ActionStateMachineStates::End,

                _ => {}
            },
            Event::Text(text) => {
                self.action.d_bit_text.push_str(text.to_string().as_str());
            }
            _ => {}
        }
        self
    }

    fn yaml(mut self, event: &Event) -> Self {
        match event {
            Event::Text(text) => {
                let action_yaml: ActionYaml =
                    serde_yml::from_str(&text.to_string()).expect("failed to parse yaml config");

                self.action.affects_action = action_yaml.affects_action;
                if let Some(d_bit) = action_yaml.d_bit {
                    self.action.d_bit = d_bit;
                } else {
                    self.action.d_bit = true;
                }

                if let Some(enabled) = action_yaml.enabled {
                    self.action.enabled = enabled;
                } else {
                    self.action.enabled = true;
                }
                if let Some(revertable) = action_yaml.revertable {
                    self.action.revertable = revertable;
                } else {
                    self.action.revertable = false;
                }

                self.action.ttype = action_yaml.ttype;

                self.state = ActionStateMachineStates::End
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

            let room_id = calculate_room_id(&file_path.to_str());
            let mut state_machine = RoomStateMachine::new(room_id);
            println!("= = = =");
            println!("ROOM {:?}", state_machine.state.clone());
            println!("= = = =");

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
            }

            if let Some(obj_sm) = state_machine.current_object {
                if let Some(ref mut vector) = state_machine.room.objects {
                    vector.push(obj_sm.object);
                }
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

fn calculate_room_id<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn calculate_object_id(room_id: u64, index: u64) -> u64 {
    let mut s = DefaultHasher::new();
    room_id.hash(&mut s);
    index.hash(&mut s);
    s.finish()
}

fn calculate_action_id(object_id: u64, index: u64) -> u64 {
    let mut s = DefaultHasher::new();
    object_id.hash(&mut s);
    index.hash(&mut s);
    s.finish()
}
