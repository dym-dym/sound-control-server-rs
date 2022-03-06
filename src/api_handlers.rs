use serde::{Serialize, Deserialize};
use actix_web::{web, Responder, HttpResponse};
use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVolumeInfo {
    volume: u8,
    id: String,
    muted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    time: u64,
    err: String,
}

pub async fn upload() -> impl Responder {
    let u = &File{
        name: "dummy data".to_string(),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        err: "".to_string(),
    };

    HttpResponse::Ok().json(u)
}

pub async fn update_volume(instr: web::Json<AppVolumeInfo>) -> HttpResponse {
    println!("instruction : {:?}", instr);
    println!("volume: {}", instr.volume);
    println!("id: {}", instr.id);
    println!("muted: {}", instr.muted);

    HttpResponse::Ok().json(instr.0)
}

pub async fn request_infos() -> impl Responder {
    let info = volume::get_app_info();
}

// When called tries to get the JSON content of the request and display it
//pub async fn download(info: web::Json<File>) -> HttpResponse {

    //println!("{:?}", info); // Prints the JSON content to the console as it has been deserialized into a File struct

    //HttpResponse::Ok().json(info.0) // Sends back a code 200 HttpResponse with the JSON as its content
//}
