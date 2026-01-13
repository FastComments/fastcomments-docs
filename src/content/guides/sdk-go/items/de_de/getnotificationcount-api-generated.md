## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| urlId | string | query | Nein |  |
| fromCommentId | string | query | Nein |  |
| viewed | boolean | query | Nein |  |
| type | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notification_count_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetNotificationCount Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optional)
	urlId := "urlId_example" // string |  (optional)
	fromCommentId := "fromCommentId_example" // string |  (optional)
	viewed := true // bool |  (optional)
	type_ := "type__example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotificationCount(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetNotificationCount`: GetNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotificationCount`: %v\n", resp)
}
[inline-code-end]