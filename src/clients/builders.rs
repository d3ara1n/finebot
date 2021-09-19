use std::sync::Arc;

use crate::clients::application::{Finebot, FinebotOptions};
use crate::clients::middlewares::{Middleware, Pipeline};
use crate::events::GenericEvent;

pub struct FinebotBuilder
{
    host: Option<String>,
    port: Option<u16>,
    access_token: Option<String>,
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl FinebotBuilder
{
    pub fn new() -> Self
    {
        Self
        {
            host: None,
            port: None,
            access_token: None,
            middlewares: Vec::new(),
        }
    }

    pub fn build<'a>(self) -> Result<Finebot, String>
    {
        let next: Pipeline = Pipeline::new(self.middlewares, 0);

        Ok(Finebot::new(FinebotOptions
                        {
                            host: self.host.unwrap(),
                            port: self.port.unwrap(),
                            access_token: self.access_token.unwrap(),
                        },
                        next))
    }

    pub fn host(mut self, host: String) -> Self
    {
        self.host = Some(host);
        self
    }

    pub fn port(mut self, port: u16) -> Self
    {
        self.port = Some(port);
        self
    }

    pub fn access_token(mut self, access_token: String) -> Self
    {
        self.access_token = Some(access_token);
        self
    }

    pub fn middleware(mut self, middleware: impl Middleware) -> Self
    {
        self.middlewares.push(Arc::new(middleware));
        self
    }
}