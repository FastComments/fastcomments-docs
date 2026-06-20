## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nein |  |
| banEmailDomain | boolean | query | Nein |  |
| banIP | boolean | query | Nein |  |
| deleteAllUsersComments | boolean | query | Nein |  |
| bannedUntil | string | query | Nein |  |
| isShadowBan | boolean | query | Nein |  |
| updateId | string | query | Nein |  |
| banReason | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Beispiel

[inline-code-attrs-start title = 'PostBanUserFromComment Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]