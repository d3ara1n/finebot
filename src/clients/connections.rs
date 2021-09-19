use crate::events::GenericEvent;
use crate::actions::GenericAction;

pub trait Connection
{
    fn connect(&self);
    fn get(&self) -> GenericEvent;
    fn send(&self, action: GenericAction);
}