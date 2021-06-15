#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use web_chess::api::create_game;
    use web_chess::board::Action;
    use web_chess::data::GameData;

    #[actix_rt::test]
    async fn test_create_game() {
        let data = web::Data::new(GameData::new());
        let mut app =
            test::init_service(App::new().app_data(data.clone()).service(create_game)).await;
        let history: Vec<Action> = Vec::new();
        let req = test::TestRequest::post()
            .uri("/game")
            .set_json(&history)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        debug_assert!(resp.status().is_success(), "got response {}", resp.status());
    }
}
