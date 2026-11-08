## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| banEmail | boolean | query | नहीं |  |
| banEmailDomain | boolean | query | नहीं |  |
| banIP | boolean | query | नहीं |  |
| deleteAllUsersComments | boolean | query | नहीं |  |
| bannedUntil | string | query | नहीं |  |
| isShadowBan | boolean | query | नहीं |  |
| updateId | string | query | नहीं |  |
| banReason | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## उदाहरण

[inline-code-attrs-start title = 'PostBanUserFromComment उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (optional)
	banEmailDomain := true // bool |  (optional)
	banIP := true // bool |  (optional)
	deleteAllUsersComments := true // bool |  (optional)
	bannedUntil := "bannedUntil_example" // string |  (optional)
	isShadowBan := true // bool |  (optional)
	updateId := "updateId_example" // string |  (optional)
	banReason := "banReason_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).TenantId(tenantId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]