mod volume;
//use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
//use std::time::{UNIX_EPOCH, SystemTime};
use volume::AppVolumeInfo;
use pulsectl::controllers::types::ApplicationInfo;


/*
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    time: u64,
    err: String,
}

pub async fn _upload() -> impl Responder {
    let u = &File{
        name: "dummy data".to_string(),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        err: "".to_string(),
    };

    HttpResponse::Ok().json(u)
}
*/

pub async fn update_volume(instr: web::Json<AppVolumeInfo>) -> HttpResponse {
    println!("instruction : {:?}", instr);
    println!("volume: {}", instr.volume);
    println!("id: {}", instr.id);
    println!("muted: {}", instr.muted);

    match volume::update_app_volume(instr.volume, instr.id.clone(), instr.muted){
        Err(x) if x == "App not found" => HttpResponse::InternalServerError().json(x),
        Err(x) if x == "Wrong volume value" => HttpResponse::InternalServerError().json(x),
        Ok(()) => HttpResponse::Ok().json("{}"),
        _ => HttpResponse::BadRequest().json(""),
    }
}

pub async fn request_infos() -> HttpResponse {
    let infos: Vec<ApplicationInfo> = volume::get_app_infos().unwrap();

    let app_infos: Vec<AppVolumeInfo> = infos.iter().map(|x| {
        AppVolumeInfo::new(x.volume.get()[0].0, x.name.clone().unwrap(), x.mute)
    }).collect();

    println!(" app_infos : {:?}", app_infos);

    HttpResponse::Ok().json(app_infos)
}
