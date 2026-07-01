## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Returns: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_count_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetCounts Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCounts(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fout bij het aanroepen `ModerationAPI.GetCounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Volledige HTTP-respons: %v\n", r)
	}
	// respons van `GetCounts`: GetBannedUsersCountResponse
	fmt.Fprintf(os.Stdout, "Respons van `ModerationAPI.GetCounts`: %v\n", resp)
}
[inline-code-end]