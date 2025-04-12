use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use tokio::sync::RwLock;

mod handlers;
mod middleware;
mod pages;
mod words;

#[derive(Debug, Clone)]
enum LetterResult {
    CorrectPosition,
    InWordDifferentPosition,
    NotInWord,
}

impl ToString for LetterResult {
    fn to_string(&self) -> String {
        match self {
            LetterResult::CorrectPosition => "correct".to_string(),
            LetterResult::InWordDifferentPosition => "present".to_string(),
            LetterResult::NotInWord => "absent".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    guesses: Vec<Vec<(String, Vec<LetterResult>)>>,
    solved: bool,
}

#[derive(Debug, Clone)]
struct Game {
    words: Vec<String>,
    max_guesses: usize,
    created_at: Instant,
    players: HashMap<String, Player>,
}

struct AppState {
    games: Arc<RwLock<HashMap<String, Game>>>,
}

// Task that runs every hour to clean up games older than 24 hours
async fn game_cleaner(data: web::Data<AppState>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
    loop {
        interval.tick().await;
        let mut challenges = data.games.write().await;
        challenges.retain(|_, c| c.created_at.elapsed() < Duration::from_secs(86400));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        games: Arc::new(RwLock::new(HashMap::new())),
    });

    // Start the games cleaner task
    let state_clone = state.clone();
    tokio::spawn(async move {
        game_cleaner(state_clone).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(Files::new("/static", "./static").prefer_utf8(true))
            .route("/", web::get().to(pages::create_page))
            .route("/game/{game_id}", web::get().to(pages::game_page))
            .route("/create", web::post().to(handlers::create_game))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
