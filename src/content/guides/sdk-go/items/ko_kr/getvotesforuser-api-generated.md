## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |

## 응답

반환: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_for_user_response.go)

## 예제

[inline-code-attrs-start title = 'GetVotesForUser 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 문자열 | 
	urlId := "urlId_example" // 문자열 | 
	userId := "userId_example" // 문자열 |  (선택 사항)
	anonUserId := "anonUserId_example" // 문자열 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotesForUser(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetVotesForUser`의 응답: GetVotesForUserResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotesForUser`: %v\n", resp)
}
[inline-code-end]