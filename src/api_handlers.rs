mod volume;
use actix_web::{web, HttpResponse};
use volume::AppVolumeInfo;
use pulsectl::controllers::types::ApplicationInfo;


pub async fn update_volume(instr: web::Json<AppVolumeInfo>) -> HttpResponse {

    match volume::update_app_volume(instr.volume, instr.id.clone(), instr.muted){
        Err(x) => HttpResponse::InternalServerError().json(x),
        Ok(x) => HttpResponse::Ok().json(x),
    }
}

pub async fn request_infos() -> HttpResponse {
    let infos: Vec<ApplicationInfo> = volume::get_app_infos().unwrap();

    let app_infos: Vec<AppVolumeInfo> = infos.iter().map(|x| {
        AppVolumeInfo::new(x.volume.get()[0].0.try_into().unwrap(), x.name.clone().unwrap(), x.mute)
    }).collect();

    println!(" app_infos : {:?}", app_infos);

    HttpResponse::Ok().json(app_infos)
}
