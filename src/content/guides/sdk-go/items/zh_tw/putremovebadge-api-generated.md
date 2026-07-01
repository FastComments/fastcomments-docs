## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

返回：[`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_remove_user_badge_response.go)

## 範例

[inline-code-attrs-start title = 'PutRemoveBadge 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	badgeId := "badgeId_example" // string | 
	userId := "userId_example" // string |  （可選）
	commentId := "commentId_example" // string |  （可選）
	broadcastId := "broadcastId_example" // string |  （可選）
	sso := "sso_example" // string |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutRemoveBadge(context.Background()).TenantId(tenantId).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "呼叫 `ModerationAPI.PutRemoveBadge`` 時發生錯誤：%v\n", err)
		fmt.Fprintf(os.Stderr, "完整 HTTP 回應：%v\n", r)
	}
	// 來自 `PutRemoveBadge` 的回應：RemoveUserBadgeResponse
	fmt.Fprintf(os.Stdout, "來自 `ModerationAPI.PutRemoveBadge` 的回應：%v\n", resp)
}
[inline-code-end]