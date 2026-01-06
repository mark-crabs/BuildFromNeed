FROM rust:1.92.0 AS build

WORKDIR /app
COPY . .

RUN cargo build --release \
    && cargo install diesel_cli --no-default-features --features postgres


FROM rust:1.92-alpine3.20 AS final

WORKDIR /app
COPY --from=build /app/target/release/backend .
COPY --from=build /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY .env migrations/ ./ 

CMD ["sh", "-c", "diesel migration run && ./backend"]
