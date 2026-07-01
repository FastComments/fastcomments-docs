## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_award_user_badge_response.go)

## 예시

[inline-code-attrs-start title = 'PutAwardBadge 예시'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optional) // 옵션
	commentId := "commentId_example" // string |  (optional) // 옵션
	broadcastId := "broadcastId_example" // string |  (optional) // 옵션
	sso := "sso_example" // string |  (optional) // 옵션

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutAwardBadge(context.Background()).TenantId(tenantId).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.PutAwardBadge` 호출 중 오류: %v\n", err)
		fmt.Fprintf(os.Stderr, "전체 HTTP 응답: %v\n", r)
	}
	// `PutAwardBadge`에 대한 응답: AwardUserBadgeResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.PutAwardBadge`에 대한 응답: %v\n", resp)
}
[inline-code-end]