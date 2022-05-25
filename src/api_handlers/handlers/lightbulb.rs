use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightbulbInfo {
    pub intensity: i16,
    pub id: String,
    pub color: i64,
}

pub fn update_lightbulb_value(
    _intensity: i16,
    _id: String,
    _color: i64,
) -> Result<&'static str, &'static str> {
    Ok("Lightbulb {id} updated")
}
pub fn lightbulb_list() -> Vec<LightbulbInfo> {
    vec![LightbulbInfo::default()]
}

pub fn get_lightbulb_infos() -> Result<Vec<LightbulbInfo>, &'static str> {
    Ok(lightbulb_list())
}
