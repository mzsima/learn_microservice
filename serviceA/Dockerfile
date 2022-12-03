FROM golang:alpine AS builder
WORKDIR /build
COPY . .
RUN go build -o serviceA greeter_server/main.go
FROM alpine
WORKDIR /build
COPY --from=builder /build/serviceA /build/serviceA
CMD ["./serviceA"]