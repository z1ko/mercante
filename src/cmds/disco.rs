
use crate::common::*;

use serenity::{
    framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    }, 
    model::channel::Message, prelude::*, 
    utils::MessageBuilder
};

#[group]
#[commands(disco)]
struct Playlist;

#[command]
#[sub_commands(aggiungi, rimuovi, lista)]
async fn disco(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    println!("[I] Richiesta di modifica o visualizzazione dischi utente");
    Ok(())
}
 

#[command]
#[num_args(2)]
#[aliases("compra", "acquista")]
#[example("pls disco {aggiungi|compra|acquista} <nome> <link>")]
#[description("Acquista un nuovo disco e lo aggiunge alla tua collezione")]
async fn aggiungi(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("[I] Richiesta di aggiunta dischi utente");

    // Controlla argomento nome
    let nome = match args.single::<String>() {
        Ok(value) => value,
        Err(_) => {
            println!("[W] Comando non valido!");
            return Ok(());
        }
    };

    // Controlla argomento link
    let link = match args.single::<String>() {
        Ok(value) => value,
        Err(_) => {
            println!("[W] Comando non valido!");
            return Ok(());
        }
    };

    // TODO: Scala erba gatta

    // Ottiene lock al database
    let db_lock = 
    {
        let data_read = ctx.data.read().await;
        data_read.get::<Database>()
            .expect("[E] Database non è nelle variabili globali")
            .clone()
    };

    // Aggiunge disco nel database
    {
        // Postgres non permette di memorizzare u64, quindi semplicemente
        // copiamo i bit in un i64 e facciamo finta sia un u64...
        let user_id: i64 = interpret_u64_as_i64(msg.author.id.into());

        let data_write = db_lock.write().await;
        if let Err(e) = data_write.execute(
            "INSERT INTO disco (utente, nome, link) VALUES ($1, '$2', '$3')",
            &[&user_id, &nome, &link]).await 
        {
            println!("[E] Errore inserimento nel database: {}", e);
            return Ok(());
        }
    }

    // Crea risposta ad hoc in privato
    msg.author.dm(&ctx.http, |m| {
        m.content("Sono felice di poter fare affari con te, goditelo.");
        m.embed(|e| {
            e.title(nome);
            e.description(link);
            e.footer(|f| {
                f.text("Gilda dei Mercanti - Baldassarr Gonzaga");
                f
            });
            e
        });
        m
    }).await?;

    // Prova ad eliminare il messaggio dopo averlo processato
    if let Err(e) = msg.delete(&ctx.http).await {
        println!("[W] Errore eliminazione messaggio: {}", e);
    };

    Ok(())
}

#[command]
#[num_args(1)]
#[aliases("vendi", "togli")]
#[example("pls disco {rimuovi|vendi|togli} <nome>")]
#[description("Vende un disco della tua collezione, ripagando un po' del costo iniziale")]
async fn rimuovi(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("[I] Richiesta di rimozione dischi utente");

    // Controlla argomento nome
    let nome = match args.single::<String>() {
        Ok(value) => value,
        Err(_) => {
            println!("[W] Comando non valido!");
            return Ok(());
        }
    };

    // TODO: Ottieni erba gatta

    // Ottiene lock al database
    let db_lock = 
    {
        let data_read = ctx.data.read().await;
        data_read.get::<Database>()
            .expect("[E] Database non è nelle variabili globali")
            .clone()
    };

    // Aggiunge disco nel database
    {
        // Postgres non permette di memorizzare u64, quindi semplicemente
        // copiamo i bit in un i64 e facciamo finta sia un u64...
        let user_id: i64 = interpret_u64_as_i64(msg.author.id.into());

        let data_write = db_lock.write().await;
        data_write.execute(
            "DELETE FROM disco WHERE utente = $1 AND nome = '$2'",
            &[&user_id, &nome] 
        ).await?;
    }

    // Crea risposta ad hoc in privato
    msg.author.dm(&ctx.http, |m| {
        m.content("Un vero peccato, vedrò di trovare qualcun'altro di interessato a questo...");
        m
    }).await?;

    // Prova ad eliminare il messaggio dopo averlo processato
    if let Err(e) = msg.delete(&ctx.http).await {
        println!("[W] Errore eliminazione messaggio: {:?}", e);
    }

    Ok(())
}


#[command]
#[aliases("elenca", "ls")]
#[example("pls disco {lista|elenca|ls}")]
#[description("Elenca i dischi presenti nella tua collezione")]
async fn lista(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    println!("[I] Richiesta di elencazione dischi utente");

    // Ottiene lock al database
    let db_lock = 
    {
        let data_read = ctx.data.read().await;
        data_read.get::<Database>()
            .expect("[E] Database non è nelle variabili globali")
            .clone()
    };

    // Aggiunge disco nel database
    let mut fields: Vec<(String, String, bool)> = Vec::new();
    {
        // Postgres non permette di memorizzare u64, quindi semplicemente
        // copiamo i bit in un i64 e facciamo finta sia un u64...
        let user_id: i64 = interpret_u64_as_i64(msg.author.id.into());

        let data_write = db_lock.write().await;
        let rows = data_write.query(
            "SELECT nome, link FROM disco WHERE utente = $1",
            &[&user_id] 
        ).await?;

        // Estrae dati
        for row in rows {
            
            let nome: &str = row.get(0);
            let link: &str = row.get(1);

            fields.push((nome.to_string(), link.to_string(), false));
        }
    }

    // Non ci sono dischi nella playlist
    if fields.len() == 0 {

        // Genera risposta ad hoc in privato
        msg.author.dm(&ctx.http, |m| {
            m.content("Sembra che non ci sia niente nella tua libreria...");
            m
        }).await?;

    }
    else
    {
        // Genera risposta ad hoc in privato
        msg.author.dm(&ctx.http, |m| {
            m.content("Questa è la tua collezione attuale:");
            m.embed(|e| {
                e.title("Playlist");
                e.fields(fields);
                e.footer(|f| {
                    f.text("Gilda dei Mercanti - Baldassarr Gonzaga");
                    f
                });
                e
            });
            m
        }).await?;
    }

    // Prova ad eliminare il messaggio dopo averlo processato
    if let Err(e) = msg.delete(&ctx.http).await {
        println!("[W] Errore eliminazione messaggio: {:?}", e);
    }

    Ok(())
}