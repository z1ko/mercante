
use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;
use tokio_postgres::Client;

// Permette l'inserimento dei client postgres nelle variabili globali
pub struct Database;
impl TypeMapKey for Database {
    type Value = Arc<RwLock<Client>>;
}
