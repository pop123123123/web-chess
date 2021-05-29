use actix_files::{Files, NamedFile};
use actix_web::{dev, get, App, HttpResponse, HttpServer, Responder};

const FRONTEND_PATH: &str = "./front/dist/";

#[get("/message")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(
            Files::new("/", FRONTEND_PATH)
                .index_file("index.html")
                .default_handler(|req: dev::ServiceRequest| {
                    let (http_req, _payload) = req.into_parts();
                    async {
                        let response =
                            NamedFile::open("./front/dist/index.html")?.into_response(&http_req)?;
                        Ok(dev::ServiceResponse::new(http_req, response))
                    }
                }),
        )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
