use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::game::Game;

#[derive(Clone)]
pub struct AppState {
    pub games: Arc<Mutex<HashMap<uuid::Uuid, Game>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
