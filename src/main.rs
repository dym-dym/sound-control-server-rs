use actix_web::{App, web, HttpServer};

//use regex::Regex;
use std::{env, process};

mod lib;
mod api_handlers;
mod volume;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let net_info = lib::Network::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });



    println!("");


    println!("Listening on : {}:{}", net_info.ip, net_info.port);

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/volume_change/test", web::post().to(api_handlers::upload))
                    .route("/req_apps_infos", web::post().to(api_handlers::request_infos))
                    .route("/update_volume", web::post().to(api_handlers::update_volume))
            )

    })
        .bind(net_info.ip + ":" + &net_info.port)?
        .run()
        .await
}
