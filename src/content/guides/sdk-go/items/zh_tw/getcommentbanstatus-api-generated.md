## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

返回：[`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_ban_status_response.go)

## 範例

[inline-code-attrs-start title = 'GetCommentBanStatus 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (optional) // 轉為 (可選) 等同
	sso = "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCommentBanStatus(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "呼叫 `ModerationAPI.GetCommentBanStatus`` 時發生錯誤: %v\n", err)
		fmt.Fprintf(os.Stderr, "完整 HTTP 回應: %v\n", r)
	}
	// 回應自 `GetCommentBanStatus`: GetCommentBanStatusResponse
	fmt.Fprintf(os.Stdout, "來自 `ModerationAPI.GetCommentBanStatus` 的回應: %v\n", resp)
}
[inline-code-end]