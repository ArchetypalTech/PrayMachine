use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntermediaryEffect {
    pub room: Option<String>,
    #[serde(rename = "objectID")]
    pub object_id: Option<u64>,
    #[serde(rename = "actionID")]
    pub action_id: Option<u64>,
    pub object_index: Option<u64>,
    pub action_index: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntermediaryAction {
    #[serde(rename = "actionID")]
    pub action_id: u64,
    #[serde(rename = "type")]
    pub ttype: String, // TODO enum?
    pub enabled: bool,
    pub revertable: bool,
    pub d_bit_text: String,
    pub d_bit: bool,
    pub affects_action: Option<IntermediaryEffect>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntermediaryObject {
    #[serde(rename = "objID")]
    pub obj_id: u64,
    #[serde(rename = "type")]
    pub ttype: String, // TODO enum ?
    pub material: String, // TODO Enum
    pub obj_description: String,
    pub direction: Option<String>, // TODO Enum
    pub destination: Option<String>,
    pub actions: Option<Vec<IntermediaryAction>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntermediaryRoom {
    #[serde(rename = "roomID")]
    pub room_id: u64,
    pub room_name: String,
    pub room_description: String,
    pub room_type: String,
    pub biome_type: String,
    pub objects: Option<Vec<IntermediaryObject>>,
    pub object_ids: Vec<u64>,
    pub dir_obj_ids: Vec<u64>,
}
