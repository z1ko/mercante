
-- Contiene tutti gli utenti conosciuti
CREATE TABLE IF NOT EXISTS utente (
    id BIGINT PRIMARY KEY,
    eg INT NOT NULL DEFAULT 500 CHECK(eg >= 0)
);

-- Contiene tutti i messaggi esplosivi
CREATE TABLE IF NOT EXISTS merlo (
    vittima   BIGINT,
    mandante  BIGINT,
    cmd       VARCHAR(10),
    messaggio VARCHAR(256),
    
    PRIMARY KEY(vittima, mandante)
);

-- Contiene la playlist di un utente
CREATE TABLE IF NOT EXISTS disco (
    utente BIGINT,
    nome   VARCHAR(64),
    link   VARCHAR(128) NOT NULL,

    PRIMARY KEY(utente, nome)
);