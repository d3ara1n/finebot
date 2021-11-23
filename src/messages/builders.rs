use crate::messages::{MessageChain, MessageElement};

pub struct MessageChainBuilder {
    inner: Vec<MessageElement>,
}

impl MessageChainBuilder {
    pub fn new() -> MessageChainBuilder {
        MessageChainBuilder { inner: Vec::new() }
    }

    pub fn build(self) -> MessageChain {
        return MessageChain::new(self.inner);
    }

    pub fn add(mut self, element: MessageElement) -> MessageChainBuilder {
        self.inner.push(element);

        return self;
    }

    pub fn add_plain(mut self, text: String) -> MessageChainBuilder {
        self.add(MessageElement::Plain(text))
    }

    pub fn add_at(mut self, id: u64) -> MessageChainBuilder {
        self.add(MessageElement::At(id))
    }

    pub fn add_at_all(mut self) -> MessageChainBuilder {
        self.add(MessageElement::AtAll)
    }

    pub fn add_image(mut self, url: String) -> MessageChainBuilder {
        self.add(MessageElement::Image(url))
    }
}
