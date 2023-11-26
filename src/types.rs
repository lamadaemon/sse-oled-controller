use std::string::ToString;
use derivative::Derivative;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, Bytes};

#[derive(Serialize, Deserialize, Debug, Derivative)]
pub struct ScreenHandler {
    #[serde(rename = "device-type")]
    pub device_type: String,
    pub zone: String,
    #[derivative(Default(value = "screen"))]
    pub mode: String,
    pub datas: Vec<ScreenData>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OLEDDeviceType {
    ApexSeries,
    RivalSeries,
    ArctisProWireless,
    GameDAC
}

impl Into<String> for OLEDDeviceType {
    fn into(self) -> String {
        match self {
            OLEDDeviceType::ApexSeries => "screened-128x40".to_string(),
            OLEDDeviceType::RivalSeries => "screened-128x36".to_string(),
            OLEDDeviceType::ArctisProWireless => "screened-128x48".to_string(),
            OLEDDeviceType::GameDAC => "screened-128x52".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ScreenData {
    FrameData(ScreenFrameData),
    RangeData(RangeScreenData)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ScreenFrameData {
    SingleLine(SingleLineFrameData),
    MultiLine(MultiLineFrameData),
    Image(ImageFrameData),
    DynamicImage(DynamicImageFrameData)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleLineFrameData {
    #[serde(flatten)]
    pub content: LineContent,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data_accessor_data: Option<DataAccessorData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiLineFrameData {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    pub lines: Vec<LineData>
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageFrameData {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    #[serde(rename = "image-data")]
    #[serde_as(as = "Bytes")]
    pub image_data: Vec<u8>
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicImageFrameData {
    #[serde(rename = "image-data-128x36")]
    #[serde_as(as = "Bytes")]
    pub image_data_rival: RawImageRival,
    #[serde(rename = "image-data-128x40")]
    #[serde_as(as = "Bytes")]
    pub image_data_apex: RawImageApex,
    #[serde(rename = "image-data-128x48")]
    #[serde_as(as = "Bytes")]
    pub image_data_arctis_pro: RawImageArctisProWireless,
    #[serde(rename = "image-data-128x52")]
    #[serde_as(as = "Bytes")]
    pub image_data_gamedac: RawImageGameDAC
}

pub type RawImageRival = [u8; 576];
pub type RawImageApex = [u8; 640];
pub type RawImageArctisProWireless = [u8; 768];
pub type RawImageGameDAC = [u8; 832];

// Since serde don't support long arrays, and serde_as cannot support enums with long array.
// So I have to comment out this stuff.
//
// #[derive(Debug)]
// #[serde(untagged)]
// pub enum RawImage {
//     RivalSeries(RawImageRival),
//     ApexSeries(RawImageApex),
//     ArctisProWireless(RawImageArctisProWireless),
//     GameDAC(RawImageGameDAC)
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct LineData {
    #[serde(flatten)]
    pub content: LineContent,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data_accessor_data: Option<DataAccessorData>
}

#[derive(Serialize, Deserialize, Debug, Derivative)]
pub struct FrameModifiersData {
    #[serde(rename = "length-millis")]
    #[derivative(Default(value = "0"))]
    pub length_millis: i32,
    #[serde(rename = "icon-id")]
    #[derivative(Default(value = "Icon::NoIcon"))]
    pub icon_id: Icon,
    #[derivative(Default(value = "Infinite(false)"))]
    pub repeats: Repeat
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[repr(u8)]
pub enum Icon {
    NoIcon = 0,
    HealthA = 1,
    Armor = 2,
    Ammo = 3,
    Money = 4,
    Flashbang = 5,
    KillsA = 6,
    Headshot = 7,
    Helmet = 8,
    Hunger = 10,
    Air = 11,
    Compass = 12,
    Tool = 13,
    ManaA = 14,
    Clock = 15,
    Lightning = 16,
    Backpack = 17,
    AtSymbol = 18,
    Muted = 19,
    Talking = 20,
    Connect = 21,
    Disconnect = 22,
    Music = 23,
    Play = 24,
    Pause = 25,
    CPU = 27,
    GPU = 28,
    RAM = 29,
    Assists = 30,
    CreepScore = 31,
    Dead = 32,
    Dragon = 33,
    Enemies = 35,
    GameStart = 36,
    Gold = 37,
    HealthB = 38,
    KillsB = 39,
    ManaB = 40,
    Teammates = 41,
    Timer = 42,
    Temperature = 43,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Repeat {
    Infinite(bool),
    Counts(i32)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LineContent {
    Text(TextModifierData),
    ProgressBar(ProgressBarModifierData)
}

#[derive(Serialize, Deserialize, Debug, Derivative)]
pub struct TextModifierData {
    #[serde(rename = "has-text")]
    #[derivative(Default(value = "true"))]
    pub has_text: bool,
    #[derivative(Default(value = ""))]
    pub prefix: String,
    #[derivative(Default(value = ""))]
    pub suffix: String,
    #[derivative(Default(value = "false"))]
    pub bold: bool,
    #[derivative(Default(value = "0"))]
    pub wrap: i32
}

#[derive(Serialize, Deserialize, Debug, Derivative)]
pub struct ProgressBarModifierData {
    #[serde(rename = "has-text")]
    #[derivative(Default(value = "true"))]
    pub has_progress_bar: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataAccessorData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    #[serde(rename = "context-frame-key", skip_serializing_if = "Option::is_none")]
    pub context_frame_key: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RangeScreenData {
    pub low: i32,
    pub high: i32,
    pub datas: Vec<ScreenFrameData>
}
