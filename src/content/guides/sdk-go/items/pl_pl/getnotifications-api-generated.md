## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| urlId | string | query | Nie |  |
| fromCommentId | string | query | Nie |  |
| viewed | boolean | query | Nie |  |
| type | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opcjonalne)
	urlId := "urlId_example" // string |  (opcjonalne)
	fromCommentId := "fromCommentId_example" // string |  (opcjonalne)
	viewed := true // bool |  (opcjonalne)
	type_ := "type__example" // string |  (opcjonalne)
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetNotifications`: GetNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]