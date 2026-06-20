## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Hayır | Mevcut sayfanın abone olup olmadığını belirlemek için kullanılır. |
| pageSize | integer | query | Hayır |  |
| afterId | string | query | Hayır |  |
| includeContext | boolean | query | Hayır |  |
| afterCreatedAt | integer | query | Hayır |  |
| unreadOnly | boolean | query | Hayır |  |
| dmOnly | boolean | query | Hayır |  |
| noDm | boolean | query | Hayır |  |
| includeTranslations | boolean | query | Hayır |  |
| includeTenantNotifications | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_my_notifications_response.go)

## Örnek

[inline-code-attrs-start title = 'GetUserNotifications Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Mevcut sayfanın abone olup olmadığını belirlemek için kullanılır. (isteğe bağlı)
	pageSize := int32(56) // int32 |  (isteğe bağlı)
	afterId := "afterId_example" // string |  (isteğe bağlı)
	includeContext := true // bool |  (isteğe bağlı)
	afterCreatedAt := int64(789) // int64 |  (isteğe bağlı)
	unreadOnly := true // bool |  (isteğe bağlı)
	dmOnly := true // bool |  (isteğe bağlı)
	noDm := true // bool |  (isteğe bağlı)
	includeTranslations := true // bool |  (isteğe bağlı)
	includeTenantNotifications := true // bool |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).UrlId(urlId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).IncludeTenantNotifications(includeTenantNotifications).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserNotifications`: GetMyNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]