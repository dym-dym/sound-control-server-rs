use pulsectl::controllers::{SinkController, AppControl, types::ApplicationInfo};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVolumeInfo {
    pub volume: f64,
    pub id: String,
    pub muted: bool,
}

impl AppVolumeInfo {
    pub fn new(volume: f64, id: String, muted: bool) -> AppVolumeInfo {
        AppVolumeInfo{
            volume,
            id,
            muted,
        }
    }

    pub fn _update(mut self, volume: f64, muted: bool) -> Result<AppVolumeInfo, &'static str>{
        if volume > 1.0 {
            return Err("Valeur de volume impossible");
        }

        self.volume = volume;
        self.muted = muted;

        Ok(self)
    }
}


pub fn update_app_volume(differential: f64, id: String, mute: bool) -> Result<&'static str, &'static str>{


    if differential < -1.0 || differential > 1.00 {
        return Err("Wrong volume value");
    }

    let mut handler = SinkController::create().unwrap();

    let apps: &mut Vec<ApplicationInfo> = &mut handler
        .list_applications()
        .expect("Could not get list of current applications");

    let mut device_index = 100000;
    let mut found = false;

    for app in apps {
       if app.name.clone().unwrap() == id {

           device_index = app.index;
            found = true;
        }
    }

    if handler.get_app_by_index(device_index).unwrap().mute {

        if !mute {
            match handler.set_app_mute(device_index, mute) {
                Ok(_) => return Ok("App unmuted"),
                Err(_) => return Err("Could not find app")
            }
        }
    } else {
        if mute {
            match handler.set_app_mute(device_index, mute) {
                Ok(_) => return Ok("App muted"),
                Err(_) => return Err("Could not find app")
            }
        }
    }

    if differential > 0.0 {
        handler.increase_app_volume_by_percent(device_index, differential);
    }
    else if differential < 0.0 {
        handler.decrease_app_volume_by_percent(device_index, -differential)
    }

    if !found {
        return Err("App not found");
    }

    Ok("Volume set")
}

pub fn get_app_infos() -> Result<Vec<ApplicationInfo>, &'static str> {

    let mut handler = SinkController::create().unwrap();

    let apps = handler
        .list_applications()
        .expect("Could not get list of current applications");

    Ok(apps)
}
