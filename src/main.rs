use crate::messages::builders::MessageChainBuilder;

mod messages;

fn main()
{
    println!("============DEBUG==========");

    let mut builder = MessageChainBuilder::new();
    let chain = builder.add_at(666).add_plain(String::from("Hello, World")).build();


    println!("{}", chain);
}