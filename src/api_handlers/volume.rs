use pulsectl::controllers::{SinkController, AppControl, types::ApplicationInfo};
use serde::{Serialize, Deserialize};
use libpulse_binding::volume::Volume;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVolumeInfo {
    pub volume: u32,
    pub id: String,
    pub muted: bool,
}

impl AppVolumeInfo {
    pub fn new(volume: u32, id: String, muted: bool) -> AppVolumeInfo {
        AppVolumeInfo{
            volume,
            id,
            muted,
        }
    }

    pub fn _update(mut self, volume: u32, muted: bool) -> Result<AppVolumeInfo, &'static str>{
        if volume > 100 {
            return Err("Valeur de volume impossible");
        }

        self.volume = volume;
        self.muted = muted;

        Ok(self)
    }
}


pub fn update_app_volume(volume: u32, id: String, mute: bool) -> Result<(), &'static str>{

    let volume = Volume{
        0 : volume,
    };

    if !volume.is_valid(){
        return Err("Wrong volume value");
    }

    let mut handler = SinkController::create().unwrap();

    let apps: &mut Vec<ApplicationInfo> = &mut handler
        .list_applications()
        .expect("Could not get list of current applications");

    let mut found = false;

    for app in apps {
        if app.name.clone().unwrap() == id {
            println!("{:?}", app.volume.get_mut());

            if mute {
                app.volume.mute(2);
                return Ok(());
            }

            app.volume.increase(volume);
            //app.volume.set(2, volume);

            println!("{:?}", app.volume.get_mut());

            found = true;
        }
    }

    if !found {
        return Err("App not found");
    }

    Ok(())
}

pub fn get_app_infos() -> Result<Vec<ApplicationInfo>, &'static str> {

    let mut handler = SinkController::create().unwrap();

    let apps = handler
        .list_applications()
        .expect("Could not get list of current applications");

    Ok(apps)
}
