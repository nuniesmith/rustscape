//! Rustscape - Simplified single-binary game server
//!
//! Run with: cargo run
//! Connect at: http://localhost:8080

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

mod game;
mod net;
mod world;

use game::GameState;

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("rustscape=debug,tower_http=debug")
        .init();

    // Load game data from JSON files
    let game_state = Arc::new(GameState::load_from_files());
    info!("Loaded {} item definitions", game_state.items.len());
    info!("Loaded {} NPC definitions", game_state.npc_defs.len());

    // Broadcast channel for game events (chat, etc)
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app_state = AppState {
        game: game_state.clone(),
        broadcast: tx,
    };

    // Start game tick loop
    let tick_state = game_state.clone();
    tokio::spawn(async move {
        game::tick_loop(tick_state).await;
    });

    // Build router
    let app = Router::new()
        // WebSocket endpoint for game
        .route("/ws", get(ws_handler))
        // API endpoints (optional, for debugging)
        .route("/api/status", get(status_handler))
        // Serve client files from ./client/dist
        .nest_service("/", ServeDir::new("client/dist").precompressed_br())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🎮 Rustscape server running at http://localhost:8080");
    info!("   WebSocket endpoint: ws://localhost:8080/ws");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    game: Arc<GameState>,
    broadcast: broadcast::Sender<String>,
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    // Delegate to net module
    net::handle_connection(socket, state.game, state.broadcast).await;
}

async fn status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let players = state.game.players.len();
    format!("Players online: {}", players)
}
