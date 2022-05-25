use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DoorInfo {
    pub open: bool,
    pub id: String,
}

pub fn update_door_value(_open: bool, _id: String) -> Result<&'static str, &'static str> {
    Ok("Door {id} updated")
}

pub fn doors_list() -> Vec<DoorInfo> {
    vec![DoorInfo::default()]
}

pub fn get_doors_infos() -> Result<Vec<DoorInfo>, &'static str> {
    Ok(doors_list())
}
