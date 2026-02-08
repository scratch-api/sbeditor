use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Extension {
    #[serde(rename = "boost")]
    Boost,
    #[serde(rename = "ev3")]
    Ev3,
    #[serde(rename = "gdxfor")]
    Gdxfor,
    #[serde(rename = "makeymakey")]
    MakeyMakey,
    #[serde(rename = "microbit")]
    Microbit,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "text2speech")]
    Text2Speech,
    #[serde(rename = "translate")]
    Translate,
    #[serde(rename = "videosensing")]
    VideoSensing,
    #[serde(rename = "wedo2")]
    Wedo2,
    #[serde(rename = "coreExample")]
    CoreExample,
}
