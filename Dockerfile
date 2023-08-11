###############################################################################
## BUILDER
###############################################################################
FROM rust:latest as builder

# add the build targen required for alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /ribs

# Create appuser
ENV USER=ribs_user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# copy from host
COPY ./ .

# Build relase version of app 
RUN cargo build --target x86_64-unknown-linux-musl --release


###############################################################################
## RUNNER
###############################################################################
FROM alpine

LABEL Maintainer="dejan@fajfar.com"
LABEL Product="RIBS"

ENV RUST_BACKTRACE=full
ENV RUST_LOG=info

WORKDIR /ribs

# Import from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /ribs/target/x86_64-unknown-linux-musl/release/ribs ./

USER ribs_user:ribs_user

EXPOSE 7777/tcp
EXPOSE 7777/udp

CMD ["/ribs/ribs"]