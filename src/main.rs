use actix_web::{web, App, HttpServer};

//use std::{env, process};

use clap::Parser;

mod api_handlers;
mod lib;

use lib::Args;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    //let args: Vec<String> = env::args().collect();

    /*let net_info = lib::Network::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });*/

    println!("Listening on : {}:{}", args.ip, args.port);

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route(
                    "/update_volume",
                    web::post().to(api_handlers::update_volume),
                )
                .route("/request_infos", web::get().to(api_handlers::request_infos))
                .route(
                    "/request_lightbulbs",
                    web::get().to(api_handlers::request_lightbulb),
                )
                .route("/request_doors", web::get().to(api_handlers::request_doors))
                .route(
                    "/request_windows",
                    web::get().to(api_handlers::request_windows),
                )
                .route("/update_door", web::post().to(api_handlers::update_door))
                .route(
                    "/update_window",
                    web::post().to(api_handlers::update_window),
                )
                .route(
                    "/update_lightbulb",
                    web::post().to(api_handlers::update_lightbulb),
                ),
        )
    })
    .bind(args.ip + ":" + &args.port)?
    .run()
    .await
}
