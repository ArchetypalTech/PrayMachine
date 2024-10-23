use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(rename = "roomID")]
    pub room_id: Option<u64>,
    #[serde(rename = "objectID")]
    pub object_id: Option<u64>,
    #[serde(rename = "actionID")]
    pub action_id: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(rename = "actionID")]
    pub action_id: u64,
    #[serde(rename = "type")]
    pub ttype: String, // TODO enum?
    pub enabled: bool,
    pub revertable: bool,
    pub d_bit_text: String,
    pub d_bit: bool,
    // pub affects_action: Option<Effect>,
    pub affects_action: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "objID")]
    pub obj_id: u64,
    #[serde(rename = "type")]
    pub ttype: String, // TODO enum ?
    pub material: String, // TODO Enum
    pub obj_description: String,
    pub direction: Option<String>, // TODO Enu
    pub destination: Option<String>,
    pub actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename = "roomID")]
    pub room_id: u64,
    pub room_name: String,
    pub room_description: String,
    pub room_type: String,
    pub objects: Option<Vec<Object>>,
    pub object_ids: Vec<u64>,
    pub dir_obj_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub level_name: String,
    pub rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub levels: Vec<Level>,
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
