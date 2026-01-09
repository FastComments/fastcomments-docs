## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| pageSize | integer | query | Nein |  |
| afterId | string | query | Nein |  |
| includeContext | boolean | query | Nein |  |
| afterCreatedAt | integer | query | Nein |  |
| unreadOnly | boolean | query | Nein |  |
| dmOnly | boolean | query | Nein |  |
| noDm | boolean | query | Nein |  |
| includeTranslations | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (optional)
	afterId := "afterId_example" // string |  (optional)
	includeContext := true // bool |  (optional)
	afterCreatedAt := int64(789) // int64 |  (optional)
	unreadOnly := true // bool |  (optional)
	dmOnly := true // bool |  (optional)
	noDm := true // bool |  (optional)
	includeTranslations := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetUserNotifications`: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---