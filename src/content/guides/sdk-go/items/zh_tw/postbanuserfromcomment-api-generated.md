## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| banEmail | boolean | query | 否 |  |
| banEmailDomain | boolean | query | 否 |  |
| banIP | boolean | query | 否 |  |
| deleteAllUsersComments | boolean | query | 否 |  |
| bannedUntil | string | query | 否 |  |
| isShadowBan | boolean | query | 否 |  |
| updateId | string | query | 否 |  |
| banReason | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## 範例

[inline-code-attrs-start title = 'PostBanUserFromComment 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (可選)
	banEmailDomain := true // bool |  (可選)
	banIP := true // bool |  (可選)
	deleteAllUsersComments := true // bool |  (可選)
	bannedUntil := "bannedUntil_example" // string |  (可選)
	isShadowBan := true // bool |  (可選)
	updateId := "updateId_example" // string |  (可選)
	banReason := "banReason_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `PostBanUserFromComment` 的回應: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]