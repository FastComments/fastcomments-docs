## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| badgeId | string | query | はい |  |
| userId | string | query | いいえ |  |
| commentId | string | query | いいえ |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## 応答

返却: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_award_user_badge_response.go)

## 例

[inline-code-attrs-start title = 'PutAwardBadge の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (オプション)
	commentId := "commentId_example" // string |  (オプション)
	broadcastId := "broadcastId_example" // string |  (オプション)
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutAwardBadge(context.Background()).TenantId(tenantId).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "呼び出し時のエラー `ModerationAPI.PutAwardBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "完全な HTTP 応答: %v\n", r)
	}
	// `PutAwardBadge` からのレスポンス: AwardUserBadgeResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.PutAwardBadge` からのレスポンス: %v\n", resp)
}
[inline-code-end]