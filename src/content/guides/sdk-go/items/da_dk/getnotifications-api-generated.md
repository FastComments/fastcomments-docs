## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| urlId | string | query | Nej |  |
| fromCommentId | string | query | Nej |  |
| viewed | boolean | query | Nej |  |
| type | string | query | Nej |  |
| skip | number | query | Nej |  |

## Respons

Returnerer: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetNotifications Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (valgfri)
	urlId := "urlId_example" // string |  (valgfri)
	fromCommentId := "fromCommentId_example" // string |  (valgfri)
	viewed := true // bool |  (valgfri)
	type_ := "type__example" // string |  (valgfri)
	skip := float64(1.2) // float64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetNotifications`: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]