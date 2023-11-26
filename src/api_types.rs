use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::types::{Icon, ScreenHandler};

#[derive(Serialize, Deserialize, Debug)]
pub struct C2SGameCreate {
    pub game: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deinitialize_timer_length_ms: Option<u16>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2SHeartBeat {
    pub game: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2STriggerEvent {
    pub game: String,
    pub event: String,
    pub data: Option<EventData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventData {
    pub value: EventValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<HashMap<String, EventValue>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventValue {
    String(String),
    Number(i32)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2SGameRemove {
    pub game: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct C2SGameEventCreate {
    pub game: String,
    pub event: String,
    pub min_value: i32,
    pub max_value: i32,
    pub icon_id: Icon,
    pub value_optional: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2SGameEventBind {
    pub game: String,
    pub event: String,
    pub min_value: i32,
    pub max_value: i32,
    pub icon_id: Icon,
    pub handlers: Vec<ScreenHandler>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2SGameEventRemove {
    pub game: String,
    pub event: String,
}

