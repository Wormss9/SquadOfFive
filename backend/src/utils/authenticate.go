package utils

import (
	"net/http"

	gosteamauth "github.com/TeddiO/GoSteamAuth/src"
)

func Authenticate(enpoint func(http.ResponseWriter, *http.Request)) func(http.ResponseWriter, *http.Request) {

	return func(response http.ResponseWriter, request *http.Request) {
		gosteamauth.RedirectClient(response, request, gosteamauth.BuildQueryString("http://localhost:5000/api/login/steam/process"))
		return
	}
}
