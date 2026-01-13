## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Так |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |

## Response

Повертає: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_for_user_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetVotesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	userId := "userId_example" // string |  (необов'язково)
	anonUserId := "anonUserId_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotesForUser(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetVotesForUser`: GetVotesForUser200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotesForUser`: %v\n", resp)
}
[inline-code-end]