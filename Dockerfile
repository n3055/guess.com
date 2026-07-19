FROM rust:1.95

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/hello-world"]