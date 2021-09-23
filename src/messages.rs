use std::fmt::{Display, Formatter, Debug};
use crate::messages::MessageElement::{Source, Plain, At, Image, Quote};

pub mod builders;

#[derive(Debug)]
pub enum MessageElement
{
    Source(i64),
    Plain(String),
    At(u64),
    AtAll,
    Image(String),
    Quote(i64)
}

impl Display for MessageElement
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self
        {
            Source(id) => format!("[Source({})]", id),
            Plain(text) => text.to_string(),
            At(id) => format!("[at({})]", id),
            AtAll => "[atall]".to_string(),
            Image(url) => format!("[image({})]", url),
            Quote(id) =>  format!("[quote({})]", id),
            _ => "UNKNOWN".to_string()
        };
        
        write!(f, "{}", string)
    }
}

pub struct MessageChain {
    pub(crate) elements: Vec<MessageElement>,
}

impl MessageChain {
    pub fn new(elements: Vec<MessageElement>) -> MessageChain {
        MessageChain { elements }
    }
}

impl Display for MessageChain
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<String> = self.elements.iter().map( | element| -> String { element.to_string() } ).collect();
        write!(f, "{}", strings.join(""))
    }
}

impl Debug for MessageChain
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}