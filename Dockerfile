
#################################################
# BACKEND LAYER                                 #
#################################################

FROM rust AS base

# Aggiorna e installa librerie di base
RUN apt-get update

#################################################
# APPLICATION LAYER                             #
#################################################

FROM base

# Crea ambiente di lavoro
WORKDIR /app
COPY . .

# Compila e installa eseguibile
RUN cargo install --path .

CMD ["mercante"]