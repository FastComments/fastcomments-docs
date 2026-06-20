## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |

## Response

Retourneert: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_progress_list_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetUserBadgeProgressList Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optioneel)
	limit := float64(1.2) // float64 |  (optioneel)
	skip := float64(1.2) // float64 |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressList(context.Background()).TenantId(tenantId).UserId(userId).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetUserBadgeProgressList`: APIGetUserBadgeProgressListResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressList`: %v\n", resp)
}
[inline-code-end]