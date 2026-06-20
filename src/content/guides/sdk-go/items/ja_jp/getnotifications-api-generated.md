---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| userId | string | クエリ | いいえ |  |
| urlId | string | クエリ | いいえ |  |
| fromCommentId | string | クエリ | いいえ |  |
| viewed | boolean | クエリ | いいえ |  |
| type | string | クエリ | いいえ |  |
| skip | number | クエリ | いいえ |  |

## レスポンス

戻り値: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_response.go)

## 例

[inline-code-attrs-start title = 'GetNotifications の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  （オプション）
	urlId := "urlId_example" // string |  （オプション）
	fromCommentId := "fromCommentId_example" // string |  （オプション）
	viewed := true // bool |  （オプション）
	type_ := "type__example" // string |  （オプション）
	skip := float64(1.2) // float64 |  （オプション）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetNotifications` のレスポンス: GetNotificationsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]

---