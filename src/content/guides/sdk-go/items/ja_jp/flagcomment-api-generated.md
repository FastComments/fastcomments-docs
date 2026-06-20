## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| id | string | パス | はい |  |
| userId | string | クエリ | いいえ |  |
| anonUserId | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_response.go)

## 例

[inline-code-attrs-start title = 'FlagComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	userId := "userId_example" // string |  (任意)
	anonUserId := "anonUserId_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.FlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.FlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `FlagComment` の応答: FlagCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.FlagComment`: %v\n", resp)
}
[inline-code-end]