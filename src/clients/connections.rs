pub mod onebot;

use crate::events::GenericEvent;
use crate::actions::GenericAction;
use url::Url;
use async_trait::async_trait;
use futures_util::{StreamExt, Future};
use futures_util::stream::ForEach;

#[async_trait]
pub trait Connection
{
    async fn connect(&mut self);
    async fn listen<F: Fn(GenericEvent)>(&mut self, f: F);
    fn send(&mut self, action: GenericAction);
    fn new(url: Url) -> Self;
}