## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Нет | Используется для определения, подписана ли текущая страница. |
| pageSize | integer | query | Нет |  |
| afterId | string | query | Нет |  |
| includeContext | boolean | query | Нет |  |
| afterCreatedAt | integer | query | Нет |  |
| unreadOnly | boolean | query | Нет |  |
| dmOnly | boolean | query | Нет |  |
| noDm | boolean | query | Нет |  |
| includeTranslations | boolean | query | Нет |  |
| includeTenantNotifications | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Используется для определения, подписана ли текущая страница. (необязательно)
	pageSize := int32(56) // int32 |  (необязательно)
	afterId := "afterId_example" // string |  (необязательно)
	includeContext := true // bool |  (необязательно)
	afterCreatedAt := int64(789) // int64 |  (необязательно)
	unreadOnly := true // bool |  (необязательно)
	dmOnly := true // bool |  (необязательно)
	noDm := true // bool |  (необязательно)
	includeTranslations := true // bool |  (необязательно)
	includeTenantNotifications := true // bool |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]