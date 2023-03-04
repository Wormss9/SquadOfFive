FROM rust:alpine as build

WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path .

FROM alpine:latest
WORKDIR /app
COPY --from=build /usr/local/cargo/bin/backend /bin/

EXPOSE 7878
CMD ["backend"]