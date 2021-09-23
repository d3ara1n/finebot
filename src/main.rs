use std::fs::File;
use std::io::Read;
use log::info;

use finebot::clients::application::FinebotOptions;
use finebot::clients::builders::FinebotBuilder;
use simplelog::{CombinedLogger, TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

mod messages;
mod clients;
mod events;
mod relations;
mod actions;


#[tokio::main]
async fn main()
{
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Always)]).unwrap();

    info!("configuration file: {}", "config.toml");
    let mut file = File::open("config.toml").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let options: FinebotOptions = toml::from_str(&content).unwrap();
    let bot = FinebotBuilder::new()
        .host(options.host)
        .port(options.port)
        .access_token(options.access_token)
        .use_logging()
        .build().unwrap();
    bot.run().await;
}