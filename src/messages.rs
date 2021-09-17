pub mod builders;
pub mod extensions;

pub enum MessageElement
{
    Source(i64),
    Plain(String),
    At(u64),
    AtAll,
    Image(String),
    Quote(i64)
}

pub struct MessageChain
{
    pub(crate) elements: Vec<MessageElement>,
}

impl MessageChain
{
    pub fn new(def: Vec<MessageElement>) -> MessageChain
    {
        MessageChain
        {
            elements: def
        }
    }
}

impl Iterator for MessageChain
{
    type Item = MessageElement;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}