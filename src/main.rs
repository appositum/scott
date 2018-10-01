#[macro_use] extern crate serenity;

use std::env;
use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable");
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("."))
        .cmd("ping", ping));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why)
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
