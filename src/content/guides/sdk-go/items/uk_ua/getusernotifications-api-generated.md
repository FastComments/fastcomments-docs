## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Ні | Використовується для визначення, чи підписана поточна сторінка. |
| pageSize | integer | query | Ні |  |
| afterId | string | query | Ні |  |
| includeContext | boolean | query | Ні |  |
| afterCreatedAt | integer | query | Ні |  |
| unreadOnly | boolean | query | Ні |  |
| dmOnly | boolean | query | Ні |  |
| noDm | boolean | query | Ні |  |
| includeTranslations | boolean | query | Ні |  |
| includeTenantNotifications | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Використовується для визначення, чи підписана поточна сторінка. (необов'язково)
	pageSize := int32(56) // int32 |  (необов'язково)
	afterId := "afterId_example" // string |  (необов'язково)
	includeContext := true // bool |  (необов'язково)
	afterCreatedAt := int64(789) // int64 |  (необов'язково)
	unreadOnly := true // bool |  (необов'язково)
	dmOnly := true // bool |  (необов'язково)
	noDm := true // bool |  (необов'язково)
	includeTranslations := true // bool |  (необов'язково)
	includeTenantNotifications := true // bool |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]