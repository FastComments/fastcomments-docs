## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| commentId | string | 路徑 | 是 |  |
| broadcastId | string | 查詢 | 是 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`LockComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_lock_comment_200_response.go)

## 範例

[inline-code-attrs-start title = 'LockComment 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	sso := "sso_example" // string |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.LockComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.LockComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `LockComment` 的回應: LockComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.LockComment`: %v\n", resp)
}
[inline-code-end]

---