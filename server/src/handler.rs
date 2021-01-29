use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};

#[derive(Deserialize, Debug)]
pub struct ConnectRequest {
    room_id: String,
}

#[derive(Serialize, Debug)]
pub struct ConnectResponse {
    room_id: String,
    token: String,
}

pub async fn register_handler(body: ConnectRequest, clients: Clients) -> Result<impl Reply> {
    let uuid = Uuid::new_v4();

    register_client(uuid, body.room_id.clone(), clients).await;
    Ok(json(&ConnectResponse {
        room_id: body.room_id,
        token: "isak".to_string(),
    }))
}

async fn register_client(user_id: Uuid, room_id: String, clients: Clients) {
    clients.write().await.insert(
        user_id.clone(),
        Client {
            user_id,
            room_id,
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: Uuid, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: Uuid, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
