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
    vec![WindowInfo::default()]
}

pub fn update_window_value(_open: bool, _id: String) -> Result<&'static str, &'static str> {
    Ok("Window {id} updated")
}

pub fn get_windows_infos() -> Result<Vec<WindowInfo>, &'static str> {
    Ok(windows_list())
}
