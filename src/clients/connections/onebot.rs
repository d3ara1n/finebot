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
        let socket = TcpStream::connect(self.ws_url.as_str()).await?;
        let mut client = Client::new(socket, self.ws_url.as_str(),"");
        let (mut sender, mut receiver) = match client.handshake().await? {
            ServerResponse::Accepted { .. } => client.into_builder().finish(),
            ServerResponse::Redirect {status_code, location} => unimplemented!(),
            ServerResponse::Rejected {status_code} => unimplemented!()
        };
        loop {
            let data = receiver.receive_data().await;
        }
    }

    fn listen<F: Fn(GenericEvent)>(&mut self, fun: F) {
    }

    fn send(&mut self, action: GenericAction) {
    }

    fn new(url: Url) -> Self {
        Self {
            ws_url: url,
        }
    }
}
