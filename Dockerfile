FROM docker.io/rust:slim AS builder

RUN apt-get update -y && apt-get install -y musl-tools
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
	"linux/amd64") echo x86_64-unknown-linux-musl > /rust_targets ;; \
	*) exit 1 ;; \
    esac

RUN rustup target add $(cat /rust_targets)

COPY . ./app
WORKDIR /app
RUN cargo build --release --target $(cat /rust_targets)
RUN cp target/$(cat /rust_targets)/release/rusty-ip ./

FROM docker.io/busybox:latest

COPY --from=builder /app/rusty-ip /app/rusty-ip
WORKDIR /app

ENTRYPOINT ["./rusty-ip"]
