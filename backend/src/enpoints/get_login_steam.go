package enpoints

import (
	"net/http"

	gosteamauth "github.com/TeddiO/GoSteamAuth/src"
)

func GetLoginSteam(response http.ResponseWriter, request *http.Request) {
	gosteamauth.RedirectClient(response, request, gosteamauth.BuildQueryString("http://localhost:5000/api/login/steam/process"))
	return
}
