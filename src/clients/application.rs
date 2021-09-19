use serde_derive::Deserialize;
use crate::events::{GenericEvents, FriendMessageEventArgs};
use crate::clients::middlewares::{Pipeline, Middleware};
use std::sync::Arc;
use crate::events::GenericEvents::FriendMessage;
use crate::messages::MessageChain;
use crate::relations::Friend;

#[derive(Deserialize)]
pub struct FinebotOptions
{
    pub host: String,
    pub port: u16,
    pub access_token: String
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

    pub fn run(mut self)
    {
        let friend = Friend::new(0, "hello".to_string(), "world".to_string());
        let event = FriendMessageEventArgs::new(MessageChain::new(Vec::new()), friend);
        self.pipeline.run(GenericEvents::FriendMessage(event));
    }
}