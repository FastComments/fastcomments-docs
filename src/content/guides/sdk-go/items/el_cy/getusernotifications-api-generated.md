## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Όχι | Χρησιμοποιείται για να προσδιοριστεί εάν η τρέχουσα σελίδα έχει εγγραφεί. |
| pageSize | integer | query | Όχι |  |
| afterId | string | query | Όχι |  |
| includeContext | boolean | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| includeTranslations | boolean | query | Όχι |  |
| includeTenantNotifications | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Χρησιμοποιείται για να προσδιοριστεί εάν η τρέχουσα σελίδα έχει εγγραφεί. (προαιρετικό)
	pageSize := int32(56) // int32 |  (προαιρετικό)
	afterId := "afterId_example" // string |  (προαιρετικό)
	includeContext := true // bool |  (προαιρετικό)
	afterCreatedAt := int64(789) // int64 |  (προαιρετικό)
	unreadOnly := true // bool |  (προαιρετικό)
	dmOnly := true // bool |  (προαιρετικό)
	noDm := true // bool |  (προαιρετικό)
	includeTranslations := true // bool |  (προαιρετικό)
	includeTenantNotifications := true // bool |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]