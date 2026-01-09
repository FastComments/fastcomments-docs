특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| notificationId | string | 경로 | 예 |  |
| optedInOrOut | string | 경로 | 예 |  |
| commentId | string | 쿼리 | 예 |  |
| sso | string | 쿼리 | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## 예제

[inline-code-attrs-start title = 'UpdateUserNotificationCommentSubscriptionStatus 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // 문자열 | 
	notificationId := "notificationId_example" // 문자열 | 
	optedInOrOut := "optedInOrOut_example" // 문자열 | 
	commentId := "commentId_example" // 문자열 | 
	sso := "sso_example" // 문자열 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationCommentSubscriptionStatus(context.Background(), notificationId, optedInOrOut).TenantId(tenantId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateUserNotificationCommentSubscriptionStatus`의 응답: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationCommentSubscriptionStatus`: %v\n", resp)
}
[inline-code-end]