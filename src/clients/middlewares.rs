pub mod extensions;

use std::sync::Arc;

use crate::events::GenericEvent;

pub trait Middleware: 'static
{
    fn handle(&self, event: GenericEvent, next: Pipeline);
}

impl<F: 'static + for<'a> Fn(GenericEvent, Pipeline)> Middleware for F
{
    fn handle(&self, event: GenericEvent, next: Pipeline) {
        (self)(event, next);
    }
}

pub struct Pipeline
{
    all: Vec<Arc<dyn Middleware>>,
    locator: usize,
}

impl Pipeline
{
    pub fn new(all: Vec<Arc<dyn Middleware>>, locator: usize) -> Self
    {
        Self
        {
            all,
            locator
        }
    }

    pub fn run(& self, event: GenericEvent)
    {
        if self.locator < self.all.len()
        {
            let middleware = &(self.all[self.locator]);
            middleware.handle(event, Self::new(self.all.clone(), self.locator + 1));
        }
    }
}