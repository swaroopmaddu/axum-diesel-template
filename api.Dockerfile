FROM --platform=linux/amd64 rust:1.70 as build

RUN USER=root cargo new --bin axum-diesel-template
WORKDIR /axum-diesel-template

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/axum_diesel_template*
RUN cargo build --release


FROM --platform=linux/amd64 rust:1.70
RUN cargo install solana-verify
COPY --from=build /axum-diesel-template/target/release/axum-diesel-template .
CMD ["./axum-diesel-template"]