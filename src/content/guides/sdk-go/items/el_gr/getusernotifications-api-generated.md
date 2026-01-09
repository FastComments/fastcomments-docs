## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| pageSize | integer | query | Όχι |  |
| afterId | string | query | Όχι |  |
| includeContext | boolean | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| includeTranslations | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (προαιρετικό)
	afterId := "afterId_example" // string |  (προαιρετικό)
	includeContext := true // bool |  (προαιρετικό)
	afterCreatedAt := int64(789) // int64 |  (προαιρετικό)
	unreadOnly := true // bool |  (προαιρετικό)
	dmOnly := true // bool |  (προαιρετικό)
	noDm := true // bool |  (προαιρετικό)
	includeTranslations := true // bool |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetUserNotifications`: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---