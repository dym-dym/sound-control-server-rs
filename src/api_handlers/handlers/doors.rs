use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DoorInfo {
    pub open: bool,
    pub id: String,
}

pub fn update_door_value(open: bool, id: String) -> Result<&'static str, &'static str> {
    println!("{id} maintenant : {open}");

    Ok("Door {id} updated")
}

pub fn doors_list() -> Vec<DoorInfo> {
    vec![
        DoorInfo {
            open: false,
            id: String::from("Porte d'entrée"),
        },
        DoorInfo {
            open: false,
            id: String::from("Porte de derrière"),
        },
        DoorInfo {
            open: true,
            id: String::from("Porte chambre des parents"),
        },
    ]
}

pub fn get_doors_infos() -> Result<Vec<DoorInfo>, &'static str> {
    Ok(doors_list())
}
