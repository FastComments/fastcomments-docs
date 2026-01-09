Schakel meldingen voor een pagina in of uit. Wanneer gebruikers zich op een pagina abonneren, worden meldingen aangemaakt voor nieuwe root-opmerkingen, en ook

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| url | string | query | Ja |  |
| pageTitle | string | query | Ja |  |
| subscribedOrUnsubscribed | string | path | Ja |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'UpdateUserNotificationPageSubscriptionStatus Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	url := "url_example" // string | 
	pageTitle := "pageTitle_example" // string | 
	subscribedOrUnsubscribed := "subscribedOrUnsubscribed_example" // string | 
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationPageSubscriptionStatus(context.Background(), subscribedOrUnsubscribed).TenantId(tenantId).UrlId(urlId).Url(url).PageTitle(pageTitle).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationPageSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateUserNotificationPageSubscriptionStatus`: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationPageSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]