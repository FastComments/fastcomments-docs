## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| banEmail | boolean | query | לא |  |
| banEmailDomain | boolean | query | לא |  |
| banIP | boolean | query | לא |  |
| deleteAllUsersComments | boolean | query | לא |  |
| bannedUntil | string | query | לא |  |
| isShadowBan | boolean | query | לא |  |
| updateId | string | query | לא |  |
| banReason | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string |
	banEmail := true // bool |  (אופציונלי)
	banEmailDomain := true // bool |  (אופציונלי)
	banIP := true // bool |  (אופציונלי)
	deleteAllUsersComments := true // bool |  (אופציונלי)
	bannedUntil := "bannedUntil_example" // string |  (אופציונלי)
	isShadowBan := true // bool |  (אופציונלי)
	updateId := "updateId_example" // string |  (אופציונלי)
	banReason := "banReason_example" // string |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).TenantId(tenantId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]