## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Służy do określenia, czy bieżąca strona jest subskrybowana. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Służy do określenia, czy bieżąca strona jest subskrybowana. (opcjonalne)
	pageSize := int32(56) // int32 |  (opcjonalne)
	afterId := "afterId_example" // string |  (opcjonalne)
	includeContext := true // bool |  (opcjonalne)
	afterCreatedAt := int64(789) // int64 |  (opcjonalne)
	unreadOnly := true // bool |  (opcjonalne)
	dmOnly := true // bool |  (opcjonalne)
	noDm := true // bool |  (opcjonalne)
	includeTranslations := true // bool |  (opcjonalne)
	includeTenantNotifications := true // bool |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]