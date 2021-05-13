mod lobby;
mod messages;
mod start;
mod ws;

use actix::Actor;
use actix_web::{App, HttpServer};
use lobby::Lobby;
use start::start_connection as start_connection_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route) //. rename with "as" import or naming conflict
            .data(chat_server.clone()) //register the lobby
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
