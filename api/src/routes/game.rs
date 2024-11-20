use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    models::game::{Game, GameState},
    state::AppState,
};

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Failed to create game")]
    CreationError,
}

impl IntoResponse for GameError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::CreationError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
        .into_response()
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_game))
        .route("/list", get(list_games))
        .route("/join", any(join_game))
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub code: u16,
}

pub async fn create_game(
    State(ctx): State<AppState>,
) -> Result<Json<CreateGameResponse>, GameError> {
    let id = uuid::Uuid::new_v4();

    let mut games = ctx.games.lock().unwrap();

    let code = rand::thread_rng().gen_range(1000..=9999);

    games.insert(
        id,
        Game {
            code,
            players_num: 0,

            state: GameState::WaitingToStart,
            answers: vec![],

            connections: 0,
        },
    );

    Ok(Json(CreateGameResponse { code }))
}

pub async fn join_game(State(ctx): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |sock| socket_game_loop(sock, ctx))
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum GameMessage {
    Join { code: u16 },
    Echo { msg: String },
}

fn parse_msg(msg: &str) -> GameMessage {
    serde_json::from_str(msg).unwrap()
}

async fn socket_game_loop(mut socket: WebSocket, ctx: AppState) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        // Connected!
    } else {
        return;
    }

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        match msg {
            Message::Text(content) => {
                let msg = parse_msg(&content);

                match msg {
                    GameMessage::Join { code } => {
                        println!("locking games");
                        let mut games = ctx.games.lock().unwrap();
                        println!("locked games");

                        let game = games.values_mut().find(|game| game.code == code);

                        if let Some(game) = game {
                            game.players_num += 1;
                            game.connections += 1;

                            socket.send(Message::Text("joined".to_owned()));
                        } else {
                            socket.send(Message::Text("couldn't find game".to_owned()));
                        }
                    }
                    GameMessage::Echo { msg } => {
                        socket.send(Message::Text(msg)).await;
                    }
                }
            }
            _ => {}
        };
    }
}

pub async fn list_games(State(ctx): State<AppState>) -> Result<Json<Vec<Game>>, GameError> {
    let games = ctx.games.lock().unwrap();

    let list = games.values().cloned().collect();

    drop(games);

    Ok(Json(list))
}
