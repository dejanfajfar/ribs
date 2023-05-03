FROM rust

WORKDIR /usr/src/ribs
COPY . .

RUN cargo install --path .

CMD ["ribs"]