

use crate::*;
use actix_web::{cookie::Cookie, http::StatusCode};
use futures::StreamExt;
use crate::types::RustackiHttpResponse;
use appstate::AppState;
use redis::{AsyncCommands, Commands};
use models::{events::Type, resp};


/* ――――――――――――――――――――――――――――――
 ╰┈➤ realtime streaming over ws
    users can start subscribing to hoop route to stablish a 
    realtime ws channel and send realtime data like chats 
    and user status during the app to server

    once a user logged in he can subscribe to this route
    to open and maintain a new full-duplex communication 
    with a long-lived connection to interact with server
 */
#[get("/")]
pub(self) async fn index(
    req: HttpRequest,
    mut stream: web::Payload, // in ws it is possible to convert a request's Payload to a stream of ws::Message with a web::Payload and then use stream combinators to handle actual messages which is done by using ws actors
    app_state: web::Data<AppState>,
) -> RustackiHttpResponse{ 


    let mut buffer = vec![];
    while let Some(chunk) = stream.next().await{ // reading future objects from the streamer is a mutable process
        let bytes = chunk.unwrap();
        buffer.extend_from_slice(&bytes)
    }    

    todo!()

}


pub mod exports{
    pub use super::index;
}