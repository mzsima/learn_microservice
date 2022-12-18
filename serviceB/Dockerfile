FROM rust:1.61.0 as builder

ENV PROTOBUF_VERSION=21.12

# protocのダウンロード
WORKDIR /tmp/protoc
RUN curl -L https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOBUF_VERSION}/protoc-${PROTOBUF_VERSION}-linux-aarch_64.zip -o protoc.zip && \
  unzip protoc.zip && \
  mv bin/* /usr/local/bin/ && \
  mv include/* /usr/local/include/

WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/serviceb-server /usr/local/bin/serviceb-server
CMD ["serviceb-server"]