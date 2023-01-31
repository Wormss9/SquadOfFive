FROM golang:latest as build

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download && go mod verify
COPY . .
RUN go build -o ./backend ./src

FROM alpine:latest
WORKDIR /app
COPY --from=build /app/backend ./
RUN mkdir /lib64 && ln -s /lib/libc.musl-x86_64.so.1 /lib64/ld-linux-x86-64.so.2
CMD ["./backend"]