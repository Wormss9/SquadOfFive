package main

import (
	"SquadOfFive/backend/src/database"
	"SquadOfFive/backend/src/enpoints"
	"database/sql"
	"fmt"
	"log"
	"os"

	"github.com/gin-gonic/gin"
	_ "github.com/lib/pq"
)

func main() {
	connStr := fmt.Sprintf("dbname=%s user=%s password=%s sslmode=disable",
		os.Getenv("POSTGRES_DB"),
		os.Getenv("POSTGRES_USER"),
		os.Getenv("POSTGRES_PASSWORD"))
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal(err)
	}
	database.InitializeDabase(db)

	steamuser := new(SteamUser)
	fmt.Printf("\nFailed: %+v\n", steamuser)

	server := gin.New()
	gin.SetMode(gin.ReleaseMode) //Silence!
	server.SetTrustedProxies(nil)
	server.GET("/api/login/steam", enpoints.GetLoginSteam(db))
	server.GET("/api/login/steam/process", enpoints.GetLoginSteamProcess(db))
	server.GET("/api/status", enpoints.GetStatusHandler(db))
	server.PUT("/api/card", enpoints.PutCardHandler(db))
	server.Run()
}

type SteamUser struct {
	Response struct {
		Players []struct {
			Steamid                  string `json:"steamid"`
			communityvisibilitystate int
			profilestate             int
			personaname              string
			commentpermission        int
			profileurl               string
			avatar                   string
			avatarmedium             string
			avatarfull               string
			avatarhash               string
			lastlogoff               int
			personastate             int
			primaryclanid            string
			timecreated              int
			personastateflags        int
		} `json:"players"`
	} `json:"response"`
}

// type SteamUser struct {
// 	Response Response `json:"response"`
// }

// type Response struct {
// 	Players []Players `json:"players"`
// }

type Players struct {
	Steamid                  string `json:"steamid"`
	Communityvisibilitystate int    `json:"communityvisibilitystate"`
	Profilestate             int    `json:"profilestate"`
	Personaname              string `json:"personaname"`
	Commentpermission        int    `json:"commentpermission"`
	Profileurl               string `json:"profileurl"`
	Avatar                   string `json:"avatar"`
	Avatarmedium             string `json:"avatarmedium"`
	Avatarfull               string `json:"avatarfull"`
	Avatarhash               string `json:"avatarhash"`
	Lastlogoff               int    `json:"lastlogoff"`
	Personastate             int    `json:"personastate"`
	Primaryclanid            string `json:"primaryclanid"`
	Timecreated              int    `json:"timecreated"`
	Personastateflags        int    `json:"personastateflags"`
}
