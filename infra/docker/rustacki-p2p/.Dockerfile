# --- Build Stage ---
FROM rust:slim-bullseye as builder

ENV LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH

WORKDIR /usr/src/app
COPY . /usr/src/app

# don't forget to set this to true otherwise u'll get the following error inside the container 
# actix: OSError: [Errno 99] Cannot assign requested address
ENV SQLX_OFFLINE true

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    build-essential \
    libssl-dev \
    libpq5 \
    libpq-dev \
    git \
    ca-certificates \
    tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    git clone https://github.com/cossacklabs/themis.git && \
    cd themis && \
    make install && \
    cd .. && \
    cargo build --bin rustacki --release

# --- Runtime Stage ---
FROM debian:bullseye-slim
ARG ARCH=x86_64

# is required for any http client crate like reqwest to send to https servers
# trusted ssl certs must be in /etc/ssl/certs/ca-certificates.crt
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

# is required to execute curl command from rust code
RUN apt-get -y update && apt-get -y install curl

ENV LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
ENV DB_PASSWORD=geDteDd0Ltg2135FJYQ6rjNYHYkGQa70
ENV DB_USERNAME=postgres
ENV DB_ENGINE=postgres
ENV DB_HOST=postgres
ENV DB_PORT=5432
ENV ENVIRONMENT=prod
ENV TZ=Etc/UTC
ENV APP_USER=appuser

WORKDIR /app

# copy necessary libs from the builder stage into this final stage
# cause the final binary requires linking files or .so lilke crypto
# and postgres libs to be in its path.
COPY --from=builder /usr/src/app/target/release/rustacki /app/rustacki
COPY --from=builder /usr/local/lib/lib* /usr/local/lib/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/lib* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/lib* /lib/${ARCH}-linux-gnu/

# granting the access for the root to create any folder at runtime
RUN USER=root chown -R root:root /app

# we must copy necessary runtime files into this stage for ./rustacki
COPY ./.env /app/.env
COPY assets /app/assets

USER root

EXPOSE 2262

CMD ["./rustacki --server p2p"]