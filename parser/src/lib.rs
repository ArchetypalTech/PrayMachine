use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Exit {
    #[serde(rename = "exitID")]
    exit_id: i64,
    direction: String, // TODO Enum
    is_open: bool,
    #[serde(rename = "type")]
    ttype: String, // TODO Enum
    material: String, // TODO Enum

    enabled: bool,
    revertable: bool,
    d_bit_text: String,
    destination: String,
    description: String,

    change_by_action: Option<String>,
    change_response: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(rename = "actionID")]
    action_id: i64,
    #[serde(rename = "type")]
    ttype: String, // TODO enum?
    enabled: bool,
    revertable: bool,
    d_bit_text: String,
    d_bit: bool, // equivalent to isOpen
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "objID")]
    obj_id: i64,
    #[serde(rename = "type")]
    ttype: String, // TODO enum ?
    material: String, // TODO Enum
    obj_description: String,
    actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename = "roomID")]
    room_id: i64,
    room_name: String,
    room_description: String,
    room_type: String,
    exits: Option<Vec<Exit>>,
    objects: Option<Vec<Object>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    level_name: String,
    rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    levels: Vec<Level>,
}

pub fn parse(str: &String) -> Config {
    serde_yml::from_str(str).expect("failed to parse yaml config")
}

pub fn serialize(config: &Config) -> String {
    serde_yml::to_string(config).expect("failed to serialized")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = Config {
            levels: vec![Level {
                level_name: "test".into(),
                rooms: vec![Room {
                    room_id: 1,
                    exits: None,
                    objects: None,
                    room_description: "fdd".into(),
                    room_name: "test1".into(),
                }],
            }],
        };
        let str = serialize(&config);
        assert_eq!(parse(&str), config);
    }
}
