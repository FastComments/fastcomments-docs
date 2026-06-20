## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_from_comment_response.go)

## 예제

[inline-code-attrs-start title = 'GetBanUsersFromComment 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetBanUsersFromComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetBanUsersFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetBanUsersFromComment`의 응답: GetBannedUsersFromCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetBanUsersFromComment`: %v\n", resp)
}
[inline-code-end]