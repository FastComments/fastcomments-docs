## Параметри

| Назив | Type | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| pageSize | integer | query | Не |  |
| afterId | string | query | Не |  |
| includeContext | boolean | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| includeTranslations | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (опционо)
	afterId := "afterId_example" // string |  (опционо)
	includeContext := true // bool |  (опционо)
	afterCreatedAt := int64(789) // int64 |  (опционо)
	unreadOnly := true // bool |  (опционо)
	dmOnly := true // bool |  (опционо)
	noDm := true // bool |  (опционо)
	includeTranslations := true // bool |  (опционо)
	sso := "sso_example" // string |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserNotifications`: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]