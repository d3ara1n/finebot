use crate::messages::MessageChain;

pub enum GenericAction
{
    SendFriendMessage(FriendMessageActionArgs),
    SendGroupMessage(GroupMessageActionArgs)
}

pub struct FriendMessageActionArgs
{
    target: u64,
    message: MessageChain,
    succeed_callback: Option<fn(message_id: i64)>,
    fail_callback: Option<fn()>
}

impl FriendMessageActionArgs
{
    pub fn new(target: u64, message: MessageChain) -> Self
    {
        Self
        {
            target,
            message,
            succeed_callback: None,
            fail_callback: None
        }
    }
}

pub struct GroupMessageActionArgs
{
    target: u64,
    message: MessageChain,
    succeed_callback: fn(message_id: i64),
    fail_callback: fn()
}