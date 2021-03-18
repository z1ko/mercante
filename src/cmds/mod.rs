//
// Il mercante supporta solo pochi comandi, ma cerca di offrire
// la massima qualità:
//
// pls info:                Fai presentare il mercante
// pls storia:              Fai raccontare una storia al mercante
// pls vetrina:             Mostra gli oggetti acquistabili
// pls specchio:            Mostra informazioni personali
// pls musica:              Metti della musica
// pls merlo:               Crea una frase da dire ad una persona quando esegue comandi
//

use serenity::{framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    }, model::channel::Message, prelude::*, utils::MessageBuilder};

#[group]
#[commands(info, storia, vetrina)]
struct General; // Comandi generici

// TODO: Carica da file configurazione
const PRESENTATION: &str = "
Salve, sono Baldassarr Gonzaga, il grande mercante di Montefeltro, esponente della Gilda dei Mercanti, 
capo della grande famiglia Gonzaga, capitano di vascello e avventuriero di lungo corso, 
e oggi ti offro i miei servigi e le mie risorse, ad un modico prezzo si intende.
";

// TODO: Carica da file configurazione
const VETRINA: &str = "
Ho un sacco di acquirenti molto esigenti, quindi per ora posso darti solo questi:

Strumenti musicali autosuonanti: riescono a riprodurre qualsiasi musica desideri e senza mai stancarsi, 
dei marchingegni veramente unici, però richiedono un pò di sangue per funzionare, il tuo dovrebbe andar bene. 
Trovati nelle catacombe di Ah-Chuy-Kat, un'antica città Maya perduta nelle giungle dell'America Centrale.

Merlo di Niani: una specie di uccello con una memoria incredibile, capace di emulare la voce umana senza difficoltà, 
ricordano tutto e tutti, ma soprattutto, non perdonano. Comprato da un mercante africano in una tenda a Niani, 
nella regione del Krakan, in Guinea. Ti lascerò usarlo un po' se vuoi.

Specchio di quarzo: questo piccolo specchio ha una proprietà interessante, 
permette di vedere te stesso e gli oggetti che ti appartengono da qualsiasi angolazione e distanza desideri, 
basta solo averli in mente. Non chiedermi come faccia a capire cosa ti appartiene o no. 
L'ho acquistato da un pastore nelle alture del Kazakistan, non ne conosceva le proprietà, mi è bastata una capra.

Ricorda che l'erba gatta è una valuta accettata e molto gradita, 
direi in molti casi perfino necessaria se vuoi guadagnarti la mia approvazione.
";

#[command]
// Fai presentare il mercante
async fn info(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, PRESENTATION)
        .await?;
    
    Ok(())
}

#[command]
// Fai raccontare una storia al mercante
async fn storia(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "No, in questo momento ho altro da fare, quel Sangue di Giuda non si troverà da solo")
        .await?;

    Ok(())
}

#[command]
// Mostra gli oggetti acquistabili
async fn vetrina(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Gilda dei Mercanti - Vetrina");
            e.description("Ho un sacco di acquirenti molto esigenti, quindi per ora posso darti solo questi:");
            e.fields(vec![
                ("Strumenti musicali autosuonanti", "Riescono a riprodurre qualsiasi musica desideri e senza mai stancarsi,  dei marchingegni veramente unici, però richiedono un pò di sangue per funzionare, il tuo dovrebbe andar bene.  Trovati nelle catacombe di Ah-Chuy-Kat, un'antica città Maya perduta nelle giungle dell'America Centrale.", false),
                ("Merlo di Niani", "Una specie di uccello con una memoria incredibile, capace di emulare la voce umana senza difficoltà,  ricordano tutto e tutti, ma soprattutto, non perdonano. Comprato da un mercante africano in una tenda a Niani,  nella regione del Krakan, in Guinea. Ti lascerò usarlo un po' se vuoi.", false),
                ("Specchio di quarzo", "Questo piccolo specchio ha una proprietà interessante,  permette di vedere te stesso e gli oggetti che ti appartengono da qualsiasi angolazione e distanza desideri,  basta solo averli in mente. Non chiedermi come faccia a capire cosa ti appartiene o no.  L'ho acquistato da un pastore nelle alture del Kazakistan, non ne conosceva le proprietà, mi è bastata una capra.", false),
            ]);
            e.footer(|f| {
                f.text("Ricorda che l'erba gatta è una valuta accettata e molto gradita, direi in molti casi perfino necessaria se vuoi guadagnarti la mia approvazione.");
                f
            });
            e
        });
        m
    })
    .await?;

    Ok(())
}