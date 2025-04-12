use actix_web::{Responder, web};
use askama::Template;

use crate::{AppState, LetterResult, Player, middleware::UserId};

#[derive(Template)]
#[template(path = "create.html")]
struct CreateTemplate;

pub async fn create_page() -> impl Responder {
    let template = CreateTemplate {};
    match template.render() {
        Ok(rendered) => actix_web::HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered),
        Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Template)]
#[template(path = "game.html")]
struct GameTemplate {
    game_id: String,
    num_words: usize,
    max_guesses: usize,
    guesses: Vec<Vec<(String, Vec<LetterResult>)>>,
}

pub async fn game_page(
    game_id: web::Path<String>,
    user_id: UserId,
    data: web::Data<AppState>,
) -> impl Responder {
    let game_id = game_id.into_inner();
    let mut games = data.games.write().await;
    let game = games.get_mut(&game_id);

    if let Some(game) = game {
        let player = game
            .players
            .entry(user_id.0.clone())
            .or_insert_with(|| Player {
                guesses: Vec::new(),
                solved: false,
            });

        let template = GameTemplate {
            game_id: game_id.to_string(),
            num_words: game.words.len(),
            max_guesses: game.max_guesses,
            guesses: player.guesses.clone(),
        };

        match template.render() {
            Ok(rendered) => actix_web::HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered),
            Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
        }
    } else {
        actix_web::HttpResponse::NotFound().finish()
    }
}
