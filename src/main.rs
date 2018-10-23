#[macro_use] extern crate serenity;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Game, Ready};
use serenity::model::user::OnlineStatus;

use serenity::prelude::*;

const APPOS : u64 = 146367028968554496;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx : Context, ready : Ready) {
        let logged = "Logged in as ".to_owned()
            + &ready.user.name + "#"
            + &ready.user.discriminator.to_string()
            + " (" + &ready.user.id.to_string() + ")";

        info!("{}", logged);
        info!("{} is ready!", ready.user.name);

        let game = Game::playing("Scott Pilgrim vs. The World: The Game");
        let status = OnlineStatus::DoNotDisturb;

        ctx.set_presence(Some(game), status);
    }

    fn resume(&self, _ : Context, resume : ResumedEvent) {
        debug!("Resumed; trace: {:#?}", resume.trace);
    }
}

fn main() {
    env_logger::init();

    let token = env::var("TOKEN").expect("TOKEN environment variable");
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("."))
        .cmd("ping", ping)
        .cmd("saydel", saydel));

    if let Err(reason) = client.start() {
        error!("An error occurred while running the client: {:?}", reason)
    }
}

command!(ping(_ctx, message) {
    let _ = message.reply("Pong!");
});

command!(saydel(_ctx, message, args) {
    if message.author.id == APPOS {
    let _ = message.channel_id.say(args.rest());

    if let Err(reason) = message.delete() {
        error!("Couldn't delete message on saydel: {:#?}", reason);
    }
  }
});
