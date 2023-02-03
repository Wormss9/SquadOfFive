package enpoints

import (
	"SquadOfFive/backend/src/database"
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetStatusHandler(client *database.Database) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Writer.Header().Set("Content-Type", "application/json")
		c.Writer.WriteHeader(http.StatusOK)
		return
	}
}
