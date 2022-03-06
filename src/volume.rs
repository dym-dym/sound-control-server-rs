use pulsectl::controllers::{SinkController, DeviceControl, AppControl};

let mut handler = SinkController::create().unwrap();

let devices = handler.list_devices()
    .expect("Could not get list of playback devices.");


let apps = handler.list_applications()
    .expect("Could not get list of running applications.");

println!("Playback devices : ");
for elem in devices.clone() {
    println!("[{}] {}, Volume : {}",
             elem.index,
             elem.description.as_ref().unwrap(),
             elem.volume
    );
}

println!("\nApplications available : ");
for app in apps {
    println!("[{}] {}, Muted : {}, Volume : {}",
             app.index,
             app.name.as_ref().unwrap(),
             app.mute,
             app.volume
    );
}

pub fn get_app_infos() -> Result<Vec<AppVolumeInfo>, &'static str> {

}
