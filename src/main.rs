
use serenity::{
    prelude::*,
    async_trait,
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
};

mod cmds;

#[tokio::main]
async fn main() { run().await }

struct Mercante;

#[async_trait]
impl EventHandler for Mercante {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("[I] {} is connected!", ready.user.name);
    } 
}

// Costruisce framework dei comandi
fn create_framework() -> StandardFramework 
{
    // TODO: Usare configurazione su file

    StandardFramework::new()
        .configure(|c | c
            .with_whitespace(true)
            .prefix("pls")
        )
        .group(&cmds::GENERAL_GROUP)
}

// Vero entry point
async fn run() {

    // Carica configurazione locale
    let token = std::env::var("MERCANTE_DISCORD_TOKEN")
        .expect("[E] La variabile MERCANTE_DISCORD_TOKEN non è settata.");

    let mut client = Client::builder(&token)
        .framework(create_framework())
        .event_handler(Mercante)
        .await.expect("[E] Impossibile creare client.");

    if let Err(e) = client.start().await {
        println!("[E] Errore client: {:?}", e);
    }
}
