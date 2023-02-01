package main

import (
	"SquadOfFive/backend/src/db"
	"SquadOfFive/backend/src/enpoints"
	"log"
	"net/http"
)

func main() {
	client, err := db.GetClient()
	if err != nil {
		log.Fatal(err)
	}
	http.HandleFunc("/api/login/steam", enpoints.GetLoginSteam)
	http.HandleFunc("/api/login/steam/process", enpoints.GetLoginSteamProcess(client))
	http.HandleFunc("/api/status", enpoints.GetStatusHandler(client))
	http.HandleFunc("/api/card", enpoints.PutCardHandler(client))
	log.Fatal(http.ListenAndServe(":5000", nil))
}
