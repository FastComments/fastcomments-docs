## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |

## Respons

Returnerer: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_subscriptions_api_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetSubscriptions Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	userId := "userId_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSubscriptions(context.Background()).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSubscriptions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetSubscriptions`: GetSubscriptionsAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSubscriptions`: %v\n", resp)
}
[inline-code-end]