## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| broadcastId | string | query | 是 |  |
| editKey | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_api_delete_comment_response.go)

## 範例

[inline-code-attrs-start title = 'DeleteCommentPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字串 | 
	commentId := "commentId_example" // 字串 | 
	broadcastId := "broadcastId_example" // 字串 | 
	editKey := "editKey_example" // 字串 |  （可選）
	sso := "sso_example" // 字串 |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteCommentPublic(context.Background(), tenantId, commentId).BroadcastId(broadcastId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `DeleteCommentPublic` 的回應: PublicAPIDeleteCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteCommentPublic`: %v\n", resp)
}
[inline-code-end]