## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Svar

Returnerer: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_count_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetCounts Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCounts(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fejl ved kald af `ModerationAPI.GetCounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Fuld HTTP-svar: %v\n", r)
	}
	// svar fra `GetCounts`: GetBannedUsersCountResponse
	fmt.Fprintf(os.Stdout, "Svar fra `ModerationAPI.GetCounts`: %v\n", resp)
}
[inline-code-end]

---