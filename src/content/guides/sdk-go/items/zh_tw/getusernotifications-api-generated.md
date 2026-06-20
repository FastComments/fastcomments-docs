## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 | 用於判斷目前頁面是否已訂閱。 |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| includeTenantNotifications | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## 範例

[inline-code-attrs-start title = 'GetUserNotifications 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 用於判斷目前頁面是否已訂閱。 (選用)
	pageSize := int32(56) // int32 |  (選用)
	afterId := "afterId_example" // string |  (選用)
	includeContext := true // bool |  (選用)
	afterCreatedAt := int64(789) // int64 |  (選用)
	unreadOnly := true // bool |  (選用)
	dmOnly := true // bool |  (選用)
	noDm := true // bool |  (選用)
	includeTranslations := true // bool |  (選用)
	includeTenantNotifications := true // bool |  (選用)
	sso := "sso_example" // string |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserNotifications` 的回應：GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]