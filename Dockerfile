

#==================================================#
# Genera ricetta progetto                          #
#==================================================#

FROM rust AS planner

WORKDIR /app
COPY . .

RUN cargo install cargo-chef && \
    cargo chef prepare --recipe-path recipe.json

#==================================================#
# Compila dipendenze progetto                      #
#==================================================#

FROM rust AS cacher

WORKDIR /app
COPY . .

COPY --from=planner /app/recipe.json recipe.json
RUN cargo install cargo-chef && \
    cargo chef cook --release --recipe-path recipe.json

#==================================================#
# Compila progetto                                 #
#==================================================#

FROM rust as builder

WORKDIR /app
COPY . .

# Copia risultato degli step precedenti
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release --bin mercante

#==================================================#
# Immagine finale                                  #
#==================================================#

FROM rust as runtime

COPY --from=builder /app/target/release/mercante /usr/local/bin
COPY ./translations/initialize.sql /etc/mercante/initialize.sql

CMD ["/usr/local/bin/mercante"]