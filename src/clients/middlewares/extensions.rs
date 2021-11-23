use crate::clients::builders::FinebotBuilder;
use crate::clients::middlewares::Pipeline;
use crate::events::GenericEvent;
use log::{error, info, warn};

impl FinebotBuilder {
    pub fn use_logging(self) -> Self {
        self.middleware(|event: GenericEvent, next: Pipeline| {
            info!("{:?}", event);
            next.run(event);
        })
    }
}
