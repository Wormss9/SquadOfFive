export $(grep -v '^#' ../.env.2.local | xargs)
go run ./src/main.go