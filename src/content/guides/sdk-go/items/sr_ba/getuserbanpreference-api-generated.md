## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_moderate_get_user_ban_preferences_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetUserBanPreference'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetUserBanPreference(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Greška prilikom poziva `ModerationAPI.GetUserBanPreference``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Puni HTTP odgovor: %v\n", r)
	}
	// odgovor iz `GetUserBanPreference`: APIModerateGetUserBanPreferencesResponse
	fmt.Fprintf(os.Stdout, "Odgovor iz `ModerationAPI.GetUserBanPreference`: %v\n", resp)
}
[inline-code-end]