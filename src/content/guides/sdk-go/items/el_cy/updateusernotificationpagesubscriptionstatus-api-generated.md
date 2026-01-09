Ενεργοποιήστε ή απενεργοποιήστε ειδοποιήσεις για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις
για νέα σχόλια ρίζας, και επίσης

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |
| url | string | query | Ναι |  |
| pageTitle | string | query | Ναι |  |
| subscribedOrUnsubscribed | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα UpdateUserNotificationPageSubscriptionStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationPageSubscriptionStatus(context.Background(), subscribedOrUnsubscribed).TenantId(tenantId).UrlId(urlId).Url(url).PageTitle(pageTitle).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationPageSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `UpdateUserNotificationPageSubscriptionStatus`: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationPageSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]