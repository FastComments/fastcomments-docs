## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| pageSize | integer | query | いいえ |  |
| afterId | string | query | いいえ |  |
| includeContext | boolean | query | いいえ |  |
| afterCreatedAt | integer | query | いいえ |  |
| unreadOnly | boolean | query | いいえ |  |
| dmOnly | boolean | query | いいえ |  |
| noDm | boolean | query | いいえ |  |
| includeTranslations | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## 例

[inline-code-attrs-start title = 'GetUserNotifications の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  （オプション）
	afterId := "afterId_example" // string |  （オプション）
	includeContext := true // bool |  （オプション）
	afterCreatedAt := int64(789) // int64 |  （オプション）
	unreadOnly := true // bool |  （オプション）
	dmOnly := true // bool |  （オプション）
	noDm := true // bool |  （オプション）
	includeTranslations := true // bool |  （オプション）
	sso := "sso_example" // string |  （オプション）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserNotifications` のレスポンス: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---