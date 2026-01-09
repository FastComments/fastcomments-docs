Omogućite ili onemogućite obavještenja za stranicu. Kada su korisnici pretplaćeni na stranicu, obavještenja se kreiraju za nove komentare prvog nivoa, i takođe

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| url | string | query | Da |  |
| pageTitle | string | query | Da |  |
| subscribedOrUnsubscribed | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer UpdateUserNotificationPageSubscriptionStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationPageSubscriptionStatus(context.Background(), subscribedOrUnsubscribed).TenantId(tenantId).UrlId(urlId).Url(url).PageTitle(pageTitle).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationPageSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `UpdateUserNotificationPageSubscriptionStatus`: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationPageSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]