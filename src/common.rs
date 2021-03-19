
use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;
use tokio_postgres::Client;

// Permette l'inserimento dei client postgres nelle variabili globali
pub struct Database;
impl TypeMapKey for Database {
    type Value = Arc<RwLock<Client>>;
}

// Copia i bit nell tipo richiesto
pub fn interpret_u64_as_i64(x: u64) -> i64 {
    i64::from_ne_bytes(x.to_ne_bytes())
}

// Copia i bit nell tipo richiesto
pub fn interpret_i64_as_u64(x: i64) -> u64 {
    u64::from_ne_bytes(x.to_ne_bytes())
}