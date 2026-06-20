## Paramètres

| Name | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| urlId | string | query | Non |  |
| fromCommentId | string | query | Non |  |
| viewed | boolean | query | Non |  |
| type | string | query | Non |  |

## Réponse

Renvoie : [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notification_count_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetNotificationCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (facultatif)
	urlId := "urlId_example" // string |  (facultatif)
	fromCommentId := "fromCommentId_example" // string |  (facultatif)
	viewed := true // bool |  (facultatif)
	type_ := "type__example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotificationCount(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetNotificationCount`: GetNotificationCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotificationCount`: %v\n", resp)
}
[inline-code-end]