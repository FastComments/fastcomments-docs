현재 페이지에 온라인 상태인 뷰어: 현재 웹소켓 세션이 해당 페이지에 구독되어 있는 사람들입니다.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | 아니오 | 커서: 이전 응답의 nextAfterName 값을 전달하세요. |
| afterUserId | string | query | 아니오 | 커서 타이브레이커: 이전 응답의 nextAfterUserId 값을 전달하세요. afterName이 설정된 경우 이름 동률로 인해 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## 예제

[inline-code-attrs-start title = 'GetOnlineUsers 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterName := "afterName_example" // string | 커서: 이전 응답의 nextAfterName 값을 전달하세요. (선택 사항)
	afterUserId := "afterUserId_example" // string | 커서 동률 해소자: 이전 응답의 nextAfterUserId 값을 전달하세요. afterName이 설정된 경우 이름이 동일할 때 항목이 누락되지 않도록 필수입니다. (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOnlineUsers`의 응답: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]