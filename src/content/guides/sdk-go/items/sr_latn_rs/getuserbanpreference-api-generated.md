## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_moderate_get_user_ban_preferences_response.go)

## Primer

[inline-code-attrs-start title = 'Primer za GetUserBanPreference'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	sso := "sso_example" // string | (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetUserBanPreference(context.Background()).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetUserBanPreference``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetUserBanPreference`: APIModerateGetUserBanPreferencesResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetUserBanPreference`: %v\n", resp)
}
[inline-code-end]