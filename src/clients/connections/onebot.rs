use tokio::net::TcpStream;
use url::Url;
use async_trait::async_trait;
use soketto::handshake::{Client, ServerResponse};

use crate::{actions::GenericAction, events::GenericEvent};

use super::Connection;

pub struct OnebotConnection {
    ws_url: Url,
}

#[async_trait]
impl Connection for OnebotConnection {
    async fn connect(&mut self) {
        // yes you just get connected
    }

    fn listen<F: Fn(GenericEvent)>(&mut self, fun: F) {
        // yes you just listen to the connection
    }

    fn send(&mut self, action: GenericAction) {
        // yes you just send a message
    }

    fn new(url: Url) -> Self {
        Self {
            ws_url: url,
        }
    }
}
