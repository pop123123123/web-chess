use crate::data::{create, GameData, Id};
use actix_web::{get, post, put, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[get("/game/{game_id}")]
pub async fn get_game_info(
    data: web::Data<GameData>,
    web::Path(user_id): web::Path<Id>,
) -> impl Responder {
    let game_option = data.get(&user_id);
    let response_option = game_option.map(|b| HttpResponse::Ok().json(&(*b)));
    response_option.unwrap_or_else(|| HttpResponse::NotFound().body(""))
}

#[derive(Serialize)]
struct CreateResponse {
    id: u32,
}

// Responder
impl Responder for CreateResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        ready(Ok(HttpResponse::Ok().json(&self)))
    }
}

#[post("/game")]
pub async fn create_game(data: web::Data<GameData>) -> impl Responder {
    let id = create(&data);
    CreateResponse { id }
}

#[put("/game/{game_id}/action")]
pub async fn add_game() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
