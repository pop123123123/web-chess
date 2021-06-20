#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use web_chess::api::{
        add_action, create_game, delete_last_action, get_game_info, reset_game, test_action,
        CreateResponse,
    };
    use web_chess::board::action::Action;
    use web_chess::data::GameData;

    /// App setup
    macro_rules! setup_app {
        () => {{
            let data = web::Data::new(GameData::new());
            test::init_service(
                App::new()
                    .app_data(data.clone())
                    .service(get_game_info)
                    .service(create_game)
                    .service(test_action)
                    .service(add_action)
                    .service(delete_last_action)
                    .service(reset_game),
            )
            .await
        }};
    }

    #[actix_rt::test]
    async fn test_create_game() {
        let mut app = setup_app!();

        // create game
        let history: Vec<Action> = Vec::new();
        let req = test::TestRequest::post()
            .uri("/game")
            .set_json(&history)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        debug_assert!(
            resp.status().is_success(),
            "did not expect {}",
            resp.status()
        );
    }

    #[actix_rt::test]
    async fn test_get_game_info() {
        let mut app = setup_app!();

        // create game
        let history: Vec<Action> = Vec::new();
        let req = test::TestRequest::post()
            .uri("/game")
            .set_json(&history)
            .to_request();
        let resp: CreateResponse = test::read_response_json(&mut app, req).await;

        // get game info
        let req = test::TestRequest::get()
            .uri(&*format!("/game/{}", resp.id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        debug_assert!(
            resp.status().is_success(),
            "did not expect {}",
            resp.status()
        );
    }
}
