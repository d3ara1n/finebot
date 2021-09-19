use serde_derive::Deserialize;
use crate::events::{GenericEvent, FriendMessageEventArgs};
use crate::clients::middlewares::{Pipeline, Middleware};
use std::sync::Arc;
use crate::events::GenericEvent::FriendMessage;
use crate::messages::{MessageChain, MessageElement};
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

    }
}