package enpoints

import (
	"SquadOfFive/backend/src/db"
	"fmt"
	"io"
	"net/http"
	"net/url"

	gosteamauth "github.com/TeddiO/GoSteamAuth/src"
)

func GetLoginSteamProcess(client *db.Client) func(http.ResponseWriter, *http.Request) {
	return func(resp http.ResponseWriter, req *http.Request) {
		queryString, _ := url.ParseQuery(req.URL.RawQuery)

		// Due to ParseQuery() returning a url.Values in form map[string][]string we're going to
		// convert that data structure to map[string]string so we can validate.
		queryMap := gosteamauth.ValuesToMap(queryString)

		steamID64, isValid, err := gosteamauth.ValidateResponse(queryMap) //
		if err != nil {
			fmt.Fprintf(resp, "Failed to log in\nError: %s", err)
			return
		}

		if !isValid {
			io.WriteString(resp, "Failed to log in.")
		}

		fmt.Fprintf(resp, "Successfully logged in! %s", steamID64)
	}
}

func getName(steamID64 string) {

}

func getAvatar(steamID64 string) {

}
