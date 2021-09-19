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
    succeed_callback: fn(message_id: i64),
    fail_callback: fn()
}

pub struct GroupMessageActionArgs
{
    target: u64,
    message: MessageChain,
    succeed_callback: fn(message_id: i64),
    fail_callback: fn()
}