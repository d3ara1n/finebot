use lazy_init::Lazy;

pub enum GroupRole
{
    Member,
    Administrator,
    Owner
}

pub struct Friend
{
    identity: u64,
    nickname: String,
    remark: String
}

impl Friend
{
    pub fn new(identity: u64, nickname: String, remark: String) -> Self
    {
        Self
        {
            identity,
            nickname,
            remark
        }
    }
}

pub struct Member
{
    identity: u64,
    nickname: String,
    display_name: String,
    title: String,
    group: Box<Lazy<Group>>,
    role: GroupRole
}

pub struct Group
{
    identity: u64,
    name: String,
    owner: Lazy<Member>,
    members: Lazy<Vec<Member>>
}