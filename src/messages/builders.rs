use std::ops::Add;

use crate::messages::{MessageChain, MessageElement};

pub struct MessageChainBuilder
{
    inner: Vec<MessageElement>,
}

impl MessageChainBuilder
{
    pub fn new() -> MessageChainBuilder
    {
        MessageChainBuilder
        {
            inner: Vec::new()
        }
    }

    pub fn build(self) -> MessageChain
    {
        return MessageChain::new(self.inner);
    }

    pub fn add(mut self, element: MessageElement) -> MessageChainBuilder
    {
        self.inner.push(element);

        return self;
    }
}

