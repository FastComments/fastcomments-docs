## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
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

## 响应

返回: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## 示例

[inline-code-attrs-start title = 'PostBanUserFromComment 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (可选)
	banEmailDomain := true // bool |  (可选)
	banIP := true // bool |  (可选)
	deleteAllUsersComments := true // bool |  (可选)
	bannedUntil := "bannedUntil_example" // string |  (可选)
	isShadowBan := true // bool |  (可选)
	updateId := "updateId_example" // string |  (可选)
	banReason := "banReason_example" // string |  (可选)
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `PostBanUserFromComment` 的响应: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]