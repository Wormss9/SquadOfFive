package db

import (
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Client = mongo.Client

func GetClient() (*Client, error) {
	return mongo.NewClient(options.Client().ApplyURI("mongodb://localhost:27017"))
}
