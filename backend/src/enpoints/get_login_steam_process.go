package enpoints

import (
	"SquadOfFive/backend/src/database"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"

	gosteamauth "github.com/TeddiO/GoSteamAuth/src"
	"github.com/gin-gonic/gin"
)

func GetLoginSteamProcess(db *database.Database) gin.HandlerFunc {
	return func(c *gin.Context) {
		steamId, err := validate(*c.Request)
		if err != nil {
			fmt.Fprintf(c.Writer, "Failed to validate info\nError: %s %v", err, steamId)
			c.JSON(http.StatusInternalServerError, steamId)
			return
		}
		requestURL := fmt.Sprintf("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key=%s&steamids=%v", os.Getenv("STEAM_API_TOKEN"), steamId)
		steamuser := new(SteamUser)
		err = getJson(requestURL, steamuser)
		if err != nil {
			fmt.Fprintf(c.Writer, "Failed to parse info\nError: %s", err)
			c.JSON(http.StatusInternalServerError, "Failed to parse info")
			return
		}
		player := steamuser.Response.Players[0]
		avatar, err := getAvatar(player.Avatarhash)
		if err != nil {
			fmt.Fprintf(c.Writer, "Failed to get avatar\nError: %s", err)
			c.JSON(http.StatusInternalServerError, "Failed to get avatar")
			return
		}
		token, err := database.LoginOrCreateSteamUser(db, player.Personaname, *avatar)
		c.JSON(http.StatusOK, response{token})
	}
}

func getJson(url string, target interface{}) error {
	r, err := http.Get(url)
	if err != nil {
		return err
	}
	defer r.Body.Close()
	const name, age = "Kim", 22
	fmt.Printf("%s is %d years old.\n", name, age)

	return json.NewDecoder(r.Body).Decode(target)
}

func getAvatar(avatarHash string) (*string, error) {
	url := fmt.Sprintf("https://avatars.akamai.steamstatic.com/%s_full.jpg", avatarHash)
	res, err := http.Get(url)
	if err != nil {
		log.Fatalf("http.Get -> %v", err)
	}
	defer res.Body.Close()

	data, err := ioutil.ReadAll(res.Body)

	avatar := base64.StdEncoding.EncodeToString([]byte(data))
	if err != nil {
		return nil, err
	}
	return &avatar, nil
}

func validate(r http.Request) (*string, error) {
	queryString, _ := url.ParseQuery(r.URL.RawQuery)
	queryMap := gosteamauth.ValuesToMap(queryString)
	steamID64, isValid, err := gosteamauth.ValidateResponse(queryMap) //
	if err != nil {
		return nil, err
	}
	if !isValid {
		err = fmt.Errorf("Couldn't to verify steam account")
		return nil, err
	}
	return &steamID64, nil
}

type SteamUser struct {
	Response struct {
		Players []struct {
			Personaname string
			Avatarhash  string
		}
	}
}

type response struct {
	token string
}
