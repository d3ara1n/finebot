pub mod onebot;

use crate::actions::GenericAction;
use crate::events::GenericEvent;
use url::Url;
use async_trait::async_trait;

#[async_trait]
pub trait Connection {
    async fn connect(&mut self);
    fn listen<F: Fn(GenericEvent)>(&mut self, f: F);
    fn send(&mut self, action: GenericAction);
    fn new(url: Url) -> Self;
}
