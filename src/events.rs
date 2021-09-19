use crate::messages::MessageChain;
use chrono::{Local, DateTime};
use crate::relations::{Friend, Member, Group};

pub enum GenericEvents
{
    FriendMessage(FriendMessageEventArgs),
    GroupMessage(GroupMessageEventArgs),
    FriendRecall(FriendRecallEventArgs),
    GroupRecall(GroupRecallEventArgs)
}

pub struct FriendMessageEventArgs
{
    time: DateTime<Local>,
    message: MessageChain,
    sender: Friend
}

impl FriendMessageEventArgs
{
    pub fn new(message: MessageChain, sender: Friend) -> Self
    {
        Self
        {
            time: Local::now(),
            message,
            sender
        }
    }
}

pub struct GroupMessageEventArgs
{
    time: DateTime<Local>,
    message: MessageChain,
    sender: Member,
    group: Group
}

pub struct FriendRecallEventArgs
{
    time: DateTime<Local>,
    friend: Friend,
    message_id: i64,
}

pub struct GroupRecallEventArgs
{
    time: DateTime<Local>,
    group: Group,
    member: Member,
    operator: Member
}