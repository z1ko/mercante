
use std::{fs::File, io::Read, sync::Arc};
use tokio_postgres::{NoTls, Error};
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

mod common;
use common::Database;


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
fn create_framework() -> StandardFramework  {

    // TODO: Usare configurazione su file

    StandardFramework::new()
        .configure(|c | c
            .with_whitespace(true)
            .prefix("pls")
        )
        .group(&cmds::GENERAL_GROUP)
        .group(&cmds::disco::PLAYLIST_GROUP)
}

// Crea tabelle nel database se queste non esistono
async fn initialize_database(client: &tokio_postgres::Client, filename: &str) {

    let mut file = File::open(filename)
        .expect("[E] Impossibile aprire file di creazione sql");

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("[E] Impossibile leggere file");

    // Prova a generare schema tabelle
    if let Err(e) = client.batch_execute(&s).await {
        panic!("[E] Errore creazione tabelle: {}", e);
    }
}

// Vero entry point
async fn run() {

    // Carica configurazione locale
    let token = std::env::var("DISCORD_TOKEN")
        .expect("[E] La variabile DISCORD_TOKEN non è settata.");

    // Crea client per Discord
    let mut client = Client::builder(&token)
        .framework(create_framework())
        .event_handler(Mercante)
        .await.expect("[E] Impossibile creare client.");

    // Connette al database    
    const DB_CONFIG: &str = "host=database user=mercante dbname=mercante password=mercante";
    let (db_client, con) = tokio_postgres::connect(DB_CONFIG, NoTls)
        .await.expect("[E] Impossibile collegare database");
    
    // Spinna driver database in background
    tokio::spawn(async move {
        if let Err(e) = con.await {
            eprintln!("[E] Errore connessione database: {}", e);
        }
    });
    
    // Inizializza database con le strutture richieste
    initialize_database(&&db_client, "/etc/mercante/initialize.sql")
        .await;

    // Aggiunge variabili globali
    {
        let mut globals = client.data.write().await;
        globals.insert::<Database>(Arc::new(RwLock::new(db_client)));
    }

    // Spinna handler discord su questo thread
    if let Err(e) = client.start().await {
        println!("[E] Errore client: {:?}", e);
    }
}
