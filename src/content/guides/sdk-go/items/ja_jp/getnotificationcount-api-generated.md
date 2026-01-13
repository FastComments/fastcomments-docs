## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| urlId | string | query | いいえ |  |
| fromCommentId | string | query | いいえ |  |
| viewed | boolean | query | いいえ |  |
| type | string | query | いいえ |  |

## レスポンス

返却: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notification_count_200_response.go)

## 例

[inline-code-attrs-start title = 'GetNotificationCount の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (オプション)
	urlId := "urlId_example" // string |  (オプション)
	fromCommentId := "fromCommentId_example" // string |  (オプション)
	viewed := true // bool |  (オプション)
	type_ := "type__example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotificationCount(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// GetNotificationCount からのレスポンス: GetNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotificationCount`: %v\n", resp)
}
[inline-code-end]