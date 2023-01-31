package main

import (
	"SquadOfFive/backend/src/enpoints"
	"log"
	"net/http"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func main() {
	client, err := mongo.NewClient(options.Client().ApplyURI("mongodb://localhost:27017"))
	if err != nil {
		log.Fatal(err)
	}
	http.HandleFunc("/api/status", enpoints.GetStatusHandler(client))
	http.HandleFunc("/api/card", enpoints.PutCardHandler(client))
	log.Fatal(http.ListenAndServe(":5000", nil))
}
