## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_progress_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetUserBadgeProgressById'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressById(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetUserBadgeProgressById`: APIGetUserBadgeProgressResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressById`: %v\n", resp)
}
[inline-code-end]