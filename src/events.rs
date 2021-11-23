use crate::messages::MessageChain;
use crate::relations::{Friend, Group, Member};
use chrono::{DateTime, Local};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GenericEvent {
    FriendMessage(FriendMessageEventArgs),
    GroupMessage(GroupMessageEventArgs),
    FriendRecall(FriendRecallEventArgs),
    GroupRecall(GroupRecallEventArgs),
}

#[derive(Debug)]
pub struct FriendMessageEventArgs {
    time: DateTime<Local>,
    message: MessageChain,
    sender: Friend,
}

impl FriendMessageEventArgs {
    pub fn new(message: MessageChain, sender: Friend) -> Self {
        Self {
            time: Local::now(),
            message,
            sender,
        }
    }
}

#[derive(Debug)]
pub struct GroupMessageEventArgs {
    time: DateTime<Local>,
    message: MessageChain,
    sender: Member,
    group: Group,
}

#[derive(Debug)]
pub struct FriendRecallEventArgs {
    time: DateTime<Local>,
    friend: Friend,
    message_id: i64,
}

#[derive(Debug)]
pub struct GroupRecallEventArgs {
    time: DateTime<Local>,
    group: Group,
    member: Member,
    operator: Member,
}
