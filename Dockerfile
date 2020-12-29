FROM rustlang/rust:nightly

WORKDIR /usr/src/average_weather

EXPOSE 8000

VOLUME ["/usr/local/cargo"]

RUN cargo build --release

CMD ["cargo", "run", "--release"]
