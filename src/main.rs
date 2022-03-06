use actix_web::{App, web, HttpServer};
use pulsectl::controllers::{SinkController, DeviceControl, AppControl};

//use regex::Regex;
use std::{env, process};

mod lib;
mod api_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let _net_info = lib::Network::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

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

        //println!("\n{:?}", app);
    }

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/volume_change/test", web::post().to(api_handlers::upload))
                    .route("/volume_change", web::post().to(api_handlers::download))
            )

    })
        .bind("127.0.0.1:9000")?
        .run()
        .await

}
