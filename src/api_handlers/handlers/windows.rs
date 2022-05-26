use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WindowInfo {
    pub open: bool,
    pub id: String,
}

impl Clone for WindowInfo {
    fn clone(&self) -> Self {
        WindowInfo {
            open: self.open,
            id: self.id.clone(),
        }
    }
}

fn windows_list() -> Vec<WindowInfo> {
    vec![
        WindowInfo {
            open: false,
            id: String::from("Fenêtre salon 1"),
        },
        WindowInfo {
            open: false,
            id: String::from("Fenêtre salon 2"),
        },
        WindowInfo {
            open: true,
            id: String::from("Fenêtre chambre"),
        },
    ]
}

pub fn update_window_value(open: bool, id: String) -> Result<&'static str, &'static str> {

    println!("{id} maintenant : {open}");

    Ok("Window {id} updated")
}

pub fn get_windows_infos() -> Result<Vec<WindowInfo>, &'static str> {
    Ok(windows_list())
}
