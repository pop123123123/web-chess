use actix_files::{Files, NamedFile};
use actix_web::{dev, middleware, web, App, HttpServer};

use web_chess::api::{
    add_action, create_game, delete_last_action, get_game_info, reset_game, test_action,
};
use web_chess::data::GameData;

const FRONTEND_PATH: &str = "./front/dist/";

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
                    .service(get_game_info)
                    .service(create_game)
                    .service(test_action)
                    .service(add_action)
                    .service(delete_last_action)
                    .service(reset_game),
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
