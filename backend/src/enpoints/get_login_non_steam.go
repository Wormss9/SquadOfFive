package enpoints

import (
	"SquadOfFive/backend/src/database"

	gosteamauth "github.com/TeddiO/GoSteamAuth/src"
	"github.com/gin-gonic/gin"
)

func GetLoginNonSteam(client *database.Database) gin.HandlerFunc {
	return func(c *gin.Context) {
		gosteamauth.RedirectClient(c.Writer, c.Request, gosteamauth.BuildQueryString("http://localhost:5000/api/login/steam/process"))
	}
}
