use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightbulbInfo {
    pub intensity: i8,
    pub id: String,
    pub color: i64,
}

pub fn update_lightbulb_value(
    intensity: i16,
    id: String,
    color: i64,
) -> Result<&'static str, &'static str> {
    if lightbulb_list().is_empty() {
        return Err("No lightbulbs connected to the server");
    }

    println!("Lightbulb {id} updated with color : {color} and intensity : {intensity}");

    Ok("Lightbulb {id} updated")
}

pub fn lightbulb_list() -> Vec<LightbulbInfo> {
    vec![
        LightbulbInfo {
            intensity: 100,
            id: String::from("Salon"),
            color: 0xFFFFFF,
        },
        LightbulbInfo {
            intensity: 100,
            id: String::from("Salle de bain"),
            color: 0xFFFFFF,
        },
        LightbulbInfo {
            intensity: 100,
            id: String::from("Cuisine"),
            color: 0xFFFFFF,
        },
        LightbulbInfo {
            intensity: 100,
            id: String::from("Chambre"),
            color: 0xFFFFFF,
        },
    ]
}

pub fn get_lightbulb_infos() -> Result<Vec<LightbulbInfo>, &'static str> {
    Ok(lightbulb_list())
}
