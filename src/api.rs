use crate::board::action::ActionRequest;
use crate::board::error::InvalidMove;
use crate::data::{create, GameData, GameId};
use actix_web::dev::HttpResponseBuilder;
use actix_web::{delete, get, patch, post, put, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

// Response to the create game request
#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    pub id: GameId,
}

macro_rules! impl_respond {
    ($stru: ident) => {
        impl Responder for $stru {
            type Error = Error;
            type Future = Ready<Result<HttpResponse, Error>>;

            fn respond_to(self, _req: &HttpRequest) -> Self::Future {
                // Create response and set content type
                ready(Ok(HttpResponse::Ok().json(&self)))
            }
        }
    };
}

impl_respond!(CreateResponse);

/// Create a new game
#[post("/game")]
pub async fn create_game(
    data: web::Data<GameData>,
    history: web::Json<Vec<ActionRequest>>,
) -> impl Responder {
    let id = create(&data, history.into_inner());
    CreateResponse { id }
}

/// Get a game state
#[get("/game/{game_id}")]
pub async fn get_game_info(
    data: web::Data<GameData>,
    web::Path(game_id): web::Path<GameId>,
) -> impl Responder {
    let game_option = data.get(&game_id);
    let response_option = game_option.map(|b| HttpResponse::Ok().json(&(*b)));
    response_option.unwrap_or_else(|| {
        HttpResponse::NotFound()
            .content_type("application/json")
            .finish()
    })
}

/// Reset game
#[patch("/game/{game_id}")]
pub async fn reset_game(
    data: web::Data<GameData>,
    web::Path(game_id): web::Path<GameId>,
) -> impl Responder {
    let game_option = data.get_mut(&game_id);
    match game_option {
        Some(mut game) => {
            game.reset();
            HttpResponse::NoContent()
        }
        None => HttpResponse::NotFound(),
    }
    .finish()
}

/// Delete the last action in a game
#[delete("/game/{game_id}/last_action")]
pub async fn delete_last_action(
    data: web::Data<GameData>,
    web::Path(game_id): web::Path<GameId>,
) -> impl Responder {
    let game_option = data.get_mut(&game_id);
    match game_option {
        Some(mut game) => match game.undo_move() {
            Some(_) => HttpResponse::NoContent(),
            None => HttpResponse::BadRequest(),
        },
        None => HttpResponse::NotFound(),
    }
    .finish()
}

fn action_to_http_builder(result: &Option<InvalidMove>) -> HttpResponseBuilder {
    match result {
        Some(e) => match e {
            InvalidMove::WrongTurn => HttpResponse::Forbidden(),
            _ => HttpResponse::BadRequest(),
        },
        _ => HttpResponse::NoContent(),
    }
}

/// Test an action for a game
#[post("/game/{game_id}/action")]
pub async fn test_action(
    data: web::Data<GameData>,
    web::Path(game_id): web::Path<GameId>,
    action: web::Json<ActionRequest>,
) -> impl Responder {
    let action = action.into_inner();
    let game_option = data.get(&game_id);
    match game_option {
        Some(game) => action_to_http_builder(&game.is_move_valid(&action).err()),
        None => HttpResponse::NotFound(),
    }
    .finish()
}

/// Send an action in a game
#[put("/game/{game_id}/action")]
pub async fn add_action(
    data: web::Data<GameData>,
    web::Path(game_id): web::Path<GameId>,
    action: web::Json<ActionRequest>,
) -> impl Responder {
    let action = action.into_inner();
    let game_option = data.get_mut(&game_id);
    match game_option {
        Some(mut game) => action_to_http_builder(&game.push_move(action)),
        None => HttpResponse::NotFound(),
    }
    .finish()
}
