pub enum MessageElement
{
    Plain(String),
    At(u64),
    AtAll,
    Image(String),
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