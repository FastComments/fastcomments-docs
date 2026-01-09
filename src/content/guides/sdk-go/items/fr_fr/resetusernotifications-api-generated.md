## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| afterId | string | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de ResetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (optionnel)
	afterCreatedAt := int64(789) // int64 |  (optionnel)
	unreadOnly := true // bool |  (optionnel)
	dmOnly := true // bool |  (optionnel)
	noDm := true // bool |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `ResetUserNotifications`: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---