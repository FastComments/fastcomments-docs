## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| banEmail | boolean | query | 아니요 |  |
| banEmailDomain | boolean | query | 아니요 |  |
| banIP | boolean | query | 아니요 |  |
| deleteAllUsersComments | boolean | query | 아니요 |  |
| bannedUntil | string | query | 아니요 |  |
| isShadowBan | boolean | query | 아니요 |  |
| updateId | string | query | 아니요 |  |
| banReason | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## 예제

[inline-code-attrs-start title = 'PostBanUserFromComment 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (선택 사항)
	banEmailDomain := true // bool |  (선택 사항)
	banIP := true // bool |  (선택 사항)
	deleteAllUsersComments := true // bool |  (선택 사항)
	bannedUntil := "bannedUntil_example" // string |  (선택 사항)
	isShadowBan := true // bool |  (선택 사항)
	updateId := "updateId_example" // string |  (선택 사항)
	banReason := "banReason_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostBanUserFromComment`의 응답: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]