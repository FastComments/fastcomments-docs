## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Не |  |
| banEmailDomain | boolean | query | Не |  |
| banIP | boolean | query | Не |  |
| deleteAllUsersComments | boolean | query | Не |  |
| bannedUntil | string | query | Не |  |
| isShadowBan | boolean | query | Не |  |
| updateId | string | query | Не |  |
| banReason | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Примјер

[inline-code-attrs-start title = 'Примјер PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (необавезно)
	banEmailDomain := true // bool |  (необавезно)
	banIP := true // bool |  (необавезно)
	deleteAllUsersComments := true // bool |  (необавезно)
	bannedUntil := "bannedUntil_example" // string |  (необавезно)
	isShadowBan := true // bool |  (необавезно)
	updateId := "updateId_example" // string |  (необавезно)
	banReason := "banReason_example" // string |  (необавезно)
	sso := "sso_example" // string |  (необавезно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]

---