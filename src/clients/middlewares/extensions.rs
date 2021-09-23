use crate::clients::builders::FinebotBuilder;
use log::{info, warn, error};
use crate::clients::middlewares::Pipeline;
use crate::events::GenericEvent;

impl FinebotBuilder
{
    pub fn use_logging(self) -> Self
    {
        self.middleware(|event: GenericEvent, next: Pipeline| {
            info!("{:?}", event);
            next.run(event);
        })
    }
}