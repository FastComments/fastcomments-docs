## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwoord

Geeft terug: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetUserBadge Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadge(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// reactie van `GetUserBadge`: APIGetUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadge`: %v\n", resp)
}
[inline-code-end]