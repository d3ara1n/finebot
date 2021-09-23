use std::collections::VecDeque;

use async_trait::async_trait;
use futures_util::{FutureExt, pin_mut, SinkExt};
use futures_util::future::select;
use futures_util::stream::{ForEach, SplitSink, SplitStream, StreamExt};
use log::info;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::futures;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::actions::{FriendMessageActionArgs, GenericAction};
use crate::clients::connections::Connection;
use crate::events::{FriendMessageEventArgs, GenericEvent};
use crate::messages::MessageChain;
use crate::relations::Friend;

pub struct OnebotConnection {
    ws_url: Url,
    sender: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    reader: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

#[async_trait]
impl Connection for OnebotConnection {
    async fn connect(&mut self) {
        let (ws_stream, _) = connect_async(self.ws_url.clone()).await.unwrap();
        info!("websocket connected");
        let (mut write, read) = ws_stream.split();
        self.sender = Some(write);
        self.reader = Some(read);
        // let ws_to_stdout = read.for_each(|message| async {
        //     let data = message.unwrap().into_data();
        //     tokio::io::stdout().write_all(&data).await;
        // });
        // ws_to_stdout.await;
    }

    async fn listen<F: Fn(GenericEvent)>(&mut self, fun: F) {
        if let Some(read) = self.reader.take()
        {
            read.for_each( |message| {
                //make event
                fun(GenericEvent::FriendMessage(FriendMessageEventArgs::new(MessageChain::new(vec![]), Friend::new(0, String::new(), String::new()))));
            });
        }
    }

    fn send(&mut self, action: GenericAction) {
        if let Some(mut write) = self.sender.take()
        {
            write.send(Message::Ping(vec![]));
        }
    }

    fn new(url: Url) -> Self {
        Self {
            ws_url: url,
            sender: None,
            reader: None,
        }
    }
}
