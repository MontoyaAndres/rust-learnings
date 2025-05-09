FROM rust:latest

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo install --path .

CMD ["blog-platzi-final"]
