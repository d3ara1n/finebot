use std::sync::Arc;

use crate::events::GenericEvent;

pub trait Middleware: 'static
{
    fn handle(&self, event: GenericEvent, next: &mut Pipeline);
}

impl<F: 'static + for<'a> Fn(GenericEvent, &Pipeline)> Middleware for F
{
    fn handle(&self, event: GenericEvent, next: &mut Pipeline) {
        (self)(event, next);
    }
}

pub struct Pipeline
{
    all: Vec<Arc<dyn Middleware>>,
    next: usize
}

impl Pipeline
{
    pub fn new(all: Vec<Arc<dyn Middleware>>) -> Self
    {
        Self
        {
            all,
            next: 0
        }
    }

    pub fn run(&mut self, event: GenericEvent)
    {
        if self.next < self.all.len()
        {
            self.all[self.next].handle(event, self);
            self.next += 1;
        }
    }
}