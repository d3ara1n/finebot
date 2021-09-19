use std::fs::File;
use std::io::Read;

use finebot::clients::application::FinebotOptions;
use finebot::clients::builders::FinebotBuilder;
use finebot::clients::middlewares::Pipeline;
use finebot::events::GenericEvent;

use crate::clients::application::Finebot;
use crate::messages::builders::MessageChainBuilder;

mod messages;
mod clients;
mod events;
mod relations;

fn main()
{
    println!("============DEBUG==========");

    let mut file = File::open("config.toml").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let options: FinebotOptions = toml::from_str(&content).unwrap();

    let bot = FinebotBuilder::new()
        .host(options.host)
        .port(options.port)
        .access_token(options.access_token)
        .middleware(|event: GenericEvent, next: Pipeline| {
            println!("{:?}", event)
        })
        .middleware(|event: GenericEvent, next: Pipeline| {
            println!("[]");
            next.run(event);
            println!("[/]");
        })
        .build().unwrap();
    bot.run();
}