mod handlers;
use actix_web::{web, HttpResponse};
use handlers::doors::DoorInfo;
use handlers::lightbulb::LightbulbInfo;
use handlers::volume::AppVolumeInfo;
use handlers::windows::WindowInfo;
use pulsectl::controllers::types::ApplicationInfo;

pub async fn update_volume(instr: web::Json<AppVolumeInfo>) -> HttpResponse {
    match handlers::volume::update_app_volume(instr.volume, instr.id.clone(), instr.muted) {
        Err(x) => HttpResponse::InternalServerError().json(x),
        Ok(x) => HttpResponse::Ok().json(x),
    }
}

pub async fn update_lightbulb(instr: web::Json<LightbulbInfo>) -> HttpResponse {
    match handlers::lightbulb::update_lightbulb_value(
        instr.intensity.into(),
        instr.id.clone(),
        instr.color,
    ) {
        Err(x) => HttpResponse::InternalServerError().json(x),
        Ok(x) => HttpResponse::Ok().json(x),
    }
}

pub async fn update_door(instr: web::Json<DoorInfo>) -> HttpResponse {
    match handlers::doors::update_door_value(instr.open, instr.id.clone()) {
        Err(x) => HttpResponse::InternalServerError().json(x),
        Ok(x) => HttpResponse::Ok().json(x),
    }
}

pub async fn update_window(instr: web::Json<WindowInfo>) -> HttpResponse {
    match handlers::windows::update_window_value(instr.open, instr.id.clone()) {
        Err(x) => HttpResponse::InternalServerError().json(x),
        Ok(x) => HttpResponse::Ok().json(x),
    }
}

pub async fn request_infos() -> HttpResponse {
    let infos: Vec<ApplicationInfo> = handlers::volume::get_app_infos().unwrap();

    if infos.is_empty() {
        println!("No apps running");
        return HttpResponse::InternalServerError().json("No apps currently running");
    }

    let app_infos: Vec<AppVolumeInfo> = infos
        .iter()
        .map(|x| {
            AppVolumeInfo::new(
                x.volume.get()[0].0.try_into().unwrap(),
                x.name.clone().unwrap(),
                x.mute,
            )
        })
        .collect();

    println!(" app_infos : {:?}", app_infos);

    HttpResponse::Ok().json(app_infos)
}

pub async fn request_doors() -> HttpResponse {
    HttpResponse::Ok().json(handlers::doors::get_doors_infos().unwrap())
}

pub async fn request_windows() -> HttpResponse {
    HttpResponse::Ok().json(handlers::windows::get_windows_infos().unwrap())
}

pub async fn request_lightbulb() -> HttpResponse {
    HttpResponse::Ok().json(handlers::lightbulb::get_lightbulb_infos().unwrap())
}
