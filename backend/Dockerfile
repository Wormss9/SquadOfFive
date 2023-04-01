FROM rust:latest as build

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:latest
WORKDIR /app
COPY --from=build /usr/local/cargo/bin/backend /bin/

EXPOSE 7878
CMD ["backend"]