use std::sync::Arc;
use std::thread;
use std::time::Duration;

use log::info;
use serde_derive::Deserialize;
use url::Url;

use crate::clients::middlewares::{Middleware, Pipeline};
use crate::events::{FriendMessageEventArgs, GenericEvent};
use crate::events::GenericEvent::FriendMessage;
use crate::messages::{MessageChain, MessageElement};
use crate::relations::Friend;
use crate::clients::connections::onebot::OnebotConnection;
use crate::clients::connections::Connection;

#[derive(Deserialize)]
pub struct FinebotOptions
{
    pub host: String,
    pub port: u16,
    pub access_token: String,
}

pub struct Finebot
{
    pub options: FinebotOptions,
    pub pipeline: Pipeline,
}

impl Finebot
{
    pub fn new(options: FinebotOptions, pipeline: Pipeline) -> Finebot
    {
        Finebot
        {
            options,
            pipeline,
        }
    }

    pub async fn run(self)
    {
        let string = format!("ws://{}:{}/?access_token={}", self.options.host, self.options.port, self.options.access_token);
        let url = Url::parse(&string).unwrap();

        let mut onebot = OnebotConnection::new(url);
        info!("use onebot connection");
        onebot.connect().await;
    }
}