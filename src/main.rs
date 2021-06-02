extern crate chashmap;
extern crate rand;

use actix_files::{Files, NamedFile};
use actix_web::{dev, get, middleware, web, App, HttpResponse, HttpServer, Responder};

mod api;
mod data;
use api::{add_game, create_game, get_game_info};
use data::GameData;

const FRONTEND_PATH: &str = "./front/dist/";

/// Get welcome message
#[get("/message")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

/// Run actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3333".to_string())
        .parse()
        .expect("PORT must be a number");

    let data = web::Data::new(GameData::new());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(hello)
                    .service(get_game_info)
                    .service(create_game)
                    .service(add_game),
            )
            .service(
                Files::new("/", FRONTEND_PATH)
                    .index_file("index.html")
                    .default_handler(|req: dev::ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();
                        async {
                            let response = NamedFile::open("./front/dist/index.html")?
                                .into_response(&http_req)?;
                            Ok(dev::ServiceResponse::new(http_req, response))
                        }
                    }),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
