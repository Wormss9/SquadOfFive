package database

import (
	"database/sql"
)

const CLIENT_COLLECTION = "client"

type Database = sql.DB

func InitializeDabase(db *sql.DB) {
	db.Query(
		`CREATE TABLE IF NOT EXISTS my_user (
		id serial PRIMARY KEY,
		steam_id INT NOT NULL DEFAULT 0,
		passwd TEXT,
		name TEXT NOT NULL,
		avatar TEXT NOT NULL,
	  );`)
	db.Query(`CREATE INDEX IF NOT EXISTS steam_index ON my_user USING btree (steam_id);`)
}

func LoginOrCreateSteamUser(db *sql.DB, name string, avatar string) (string, error) {
	return "token", nil
}

// func (db *Database) Disconnect() {
// 	db.cancel()
// 	db.client.Disconnect(db.ctx)
// }

// func (db *Database) innitiate() {
// 	user := db.db.Collection("user")
// 	room := db.db.Collection("room")
// 	card := db.db.Collection("card")

// 	userIndexes := []mongo.IndexModel{
// 		mongo.IndexModel{
// 			Keys: bson.M{
// 				"id": 1,
// 			}, Options: nil,
// 		},
// 		mongo.IndexModel{
// 			Keys: bson.M{
// 				"steam_id": 1,
// 			}, Options: nil,
// 		}, mongo.IndexModel{
// 			Keys: bson.M{
// 				"name": 1,
// 			}, Options: nil,
// 		}}

// 	roomIndex := mongo.IndexModel{
// 		Keys: bson.M{
// 			"host_id": 1, // index in ascending order
// 		}, Options: nil,
// 	}

// 	cardIndexes := []mongo.IndexModel{mongo.IndexModel{
// 		Keys: bson.M{
// 			"room_host_id": 1, // index in ascending order
// 		}, Options: nil,
// 	}, mongo.IndexModel{
// 		Keys: bson.M{
// 			"player_id": 1, // index in ascending order
// 		}, Options: nil,
// 	}}

// 	user.Indexes().CreateMany(db.ctx, userIndexes)
// 	room.Indexes().CreateOne(db.ctx, roomIndex)
// 	card.Indexes().CreateMany(db.ctx, cardIndexes)

// }
