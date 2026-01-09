Aktiver eller deaktiver notifikationer for en side. Når brugere er tilmeldt en side, oprettes notifikationer for nye root-kommentarer, og også

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| url | string | query | Ja |  |
| pageTitle | string | query | Ja |  |
| subscribedOrUnsubscribed | string | path | Ja |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på UpdateUserNotificationPageSubscriptionStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationPageSubscriptionStatus(context.Background(), subscribedOrUnsubscribed).TenantId(tenantId).UrlId(urlId).Url(url).PageTitle(pageTitle).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationPageSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `UpdateUserNotificationPageSubscriptionStatus`: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationPageSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]