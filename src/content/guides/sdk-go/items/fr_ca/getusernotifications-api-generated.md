## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| pageSize | integer | query | Non |  |
| afterId | string | query | Non |  |
| includeContext | boolean | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| includeTranslations | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (optionnel)
	afterId := "afterId_example" // string |  (optionnel)
	includeContext := true // bool |  (optionnel)
	afterCreatedAt := int64(789) // int64 |  (optionnel)
	unreadOnly := true // bool |  (optionnel)
	dmOnly := true // bool |  (optionnel)
	noDm := true // bool |  (optionnel)
	includeTranslations := true // bool |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetUserNotifications` : GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]