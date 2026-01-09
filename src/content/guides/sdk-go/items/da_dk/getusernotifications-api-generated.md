## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| pageSize | integer | query | Nej |  |
| afterId | string | query | Nej |  |
| includeContext | boolean | query | Nej |  |
| afterCreatedAt | integer | query | Nej |  |
| unreadOnly | boolean | query | Nej |  |
| dmOnly | boolean | query | Nej |  |
| noDm | boolean | query | Nej |  |
| includeTranslations | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (valgfri)
	afterId := "afterId_example" // string |  (valgfri)
	includeContext := true // bool |  (valgfri)
	afterCreatedAt := int64(789) // int64 |  (valgfri)
	unreadOnly := true // bool |  (valgfri)
	dmOnly := true // bool |  (valgfri)
	noDm := true // bool |  (valgfri)
	includeTranslations := true // bool |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetUserNotifications`: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]