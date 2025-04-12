use std::{collections::HashMap, time::Instant};

use actix_web::{Responder, web};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};

use crate::{AppState, Game, Player, middleware::UserId, words};

#[derive(Serialize, Deserialize)]
pub struct CreategameForm {
    word_length: u8,
    word_count: usize,
    max_guesses: usize,
}

fn generate_code(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub async fn create_game(
    data: web::Data<AppState>,
    form: web::Form<CreategameForm>,
) -> impl Responder {
    let mut games = data.games.write().await;
    let id = generate_code(6);
    let words = words::get_random_words(form.word_length, form.word_count);
    let game = Game {
        words,
        max_guesses: form.max_guesses,
        created_at: Instant::now(),
        players: HashMap::new(),
    };
    games.insert(id.clone(), game);

    // Redirect to the game URL (HX-Redirect)
    let game_url = format!("/game/{}", id);
    actix_web::HttpResponse::SeeOther()
        .append_header(("HX-Redirect", game_url))
        .finish()
}

pub async fn make_guess(
    data: web::Data<AppState>,
    game_id: web::Path<String>,
    UserId(user_id): UserId,
    web::Json(guess): web::Json<String>,
) -> impl Responder {
    let game_id = game_id.into_inner();
    let mut games = data.games.write().await;

    if let Some(game) = games.get_mut(&game_id) {
        let mut player = game
            .players
            .entry(user_id.clone())
            .or_insert_with(|| Player {
                guesses: Vec::new(),
                solved: false,
            });
    }

    actix_web::HttpResponse::Ok()
}
