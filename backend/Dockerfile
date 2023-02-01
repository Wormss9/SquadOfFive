FROM golang:alpine as build

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download && go mod verify
COPY . .
RUN go build -o ./backend ./src

FROM alpine:latest
WORKDIR /app
COPY --from=build /app/backend ./
CMD ["./backend"]