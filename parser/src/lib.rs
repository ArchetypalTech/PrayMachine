use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(rename = "roomID")]
    room_id: Option<i64>,
    #[serde(rename = "objectID")]
    object_id: Option<i64>,
    #[serde(rename = "actionID")]
    action_id: i64,
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
    d_bit: bool,
    affects_action: Option<Effect>,
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
    direction: Option<String>, // TODO Enum
    destination: Option<String>,
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
    objects: Option<Vec<Object>>,
    object_ids: Vec<i64>,
    dir_obj_ids: Vec<i64>,
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
                    room_type: "Mountains".into(),
                    objects: None,
                    room_description: "fdd".into(),
                    room_name: "test1".into(),
                    object_ids: [].into(),
                    dir_obj_ids: [].into(),
                }],
            }],
        };
        let str = serialize(&config);
        assert_eq!(parse(&str), config);
    }
}
