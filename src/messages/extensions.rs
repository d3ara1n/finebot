use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

use crate::messages::{MessageChain, MessageElement};
use crate::messages::builders::MessageChainBuilder;

impl MessageChainBuilder
{
    pub fn add_plain(mut self, text: String) -> MessageChainBuilder
    {
        self.add(MessageElement::Plain(text))
    }

    pub fn add_at(mut self, id: u64) -> MessageChainBuilder
    {
        self.add(MessageElement::At(id))
    }

    pub fn add_at_all(mut self) -> MessageChainBuilder
    {
        self.add(MessageElement::AtAll)
    }

    pub fn add_image(mut self, url: String) -> MessageChainBuilder
    {
        self.add(MessageElement::Image(url))
    }
}

impl Display for MessageChain
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for element in &self.elements {
            let code = match element {
                MessageElement::Plain(text) => text.clone(),
                MessageElement::At(id) => format!("[at,id={}]", id),
                MessageElement::AtAll => "<AT_ALL>".to_string(),
                MessageElement::Image(file) => format!("[image,file={}]", file),

                _ => "[UNKNOWN]".to_string()
            };
            string.push_str(&code);
        }
        write!(f, "{}", string)
    }
}