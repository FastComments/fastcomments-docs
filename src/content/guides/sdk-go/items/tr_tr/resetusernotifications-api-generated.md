## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| afterId | string | sorgu | Hayır |  |
| afterCreatedAt | integer | sorgu | Hayır |  |
| unreadOnly | boolean | sorgu | Hayır |  |
| dmOnly | boolean | sorgu | Hayır |  |
| noDm | boolean | sorgu | Hayır |  |
| sso | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## Örnek

[inline-code-attrs-start title = 'ResetUserNotifications Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (isteğe bağlı)
	afterCreatedAt := int64(789) // int64 |  (isteğe bağlı)
	unreadOnly := true // bool |  (isteğe bağlı)
	dmOnly := true // bool |  (isteğe bağlı)
	noDm := true // bool |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `ResetUserNotifications`'den gelen yanıt: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]