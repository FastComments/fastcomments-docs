## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| pageSize | integer | query | Hayır |  |
| afterId | string | query | Hayır |  |
| includeContext | boolean | query | Hayır |  |
| afterCreatedAt | integer | query | Hayır |  |
| unreadOnly | boolean | query | Hayır |  |
| dmOnly | boolean | query | Hayır |  |
| noDm | boolean | query | Hayır |  |
| includeTranslations | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetUserNotifications Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (isteğe bağlı)
	afterId := "afterId_example" // string |  (isteğe bağlı)
	includeContext := true // bool |  (isteğe bağlı)
	afterCreatedAt := int64(789) // int64 |  (isteğe bağlı)
	unreadOnly := true // bool |  (isteğe bağlı)
	dmOnly := true // bool |  (isteğe bağlı)
	noDm := true // bool |  (isteğe bağlı)
	includeTranslations := true // bool |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserNotifications`'ten yanıt: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]