페이지에서 과거에 댓글을 남겼지만 현재 온라인 상태가 아닌 사용자들입니다. displayName으로 정렬됩니다.
"Members" 섹션을 렌더링하기 위해 /users/online을 모두 소진한 후 이 API를 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | No | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같은 항목이 누락되지 않도록 필요합니다. |

## Response

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## 예제

[inline-code-attrs-start title = 'GetOfflineUsers 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 페이지 URL 식별자 (서버 측에서 정리됨).
	afterName := "afterName_example" // string | 커서: 이전 응답의 nextAfterName을 전달하세요. (선택 사항)
	afterUserId := "afterUserId_example" // string | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같은 항목이 누락되지 않도록 필요합니다. (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]