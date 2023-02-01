package enpoints

import (
	"SquadOfFive/backend/src/db"
	"net/http"
)

func GetStatusHandler(client *db.Client) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		return
	}
}
