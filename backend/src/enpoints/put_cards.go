package enpoints

import (
	"net/http"

	"go.mongodb.org/mongo-driver/mongo"
)

func PutCardHandler(client *mongo.Client) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		return
	}
}
