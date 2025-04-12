use std::{collections::HashMap, sync::Arc, time::Instant};

use actix_web::web;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

mod words;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    id: u32,
    guesses: Vec<String>,
    solved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Challenge {
    id: u32,
    words: Vec<String>,
    max_guesses: usize,
    created_at: Instant,
    players: Vec<Player>,
}

struct AppState {
    challenges: Arc<RwLock<HashMap<String, Challenge>>>,
}

async fn create_challenge(
    data: web::Data<AppState>,
    form: web::Form<>
)