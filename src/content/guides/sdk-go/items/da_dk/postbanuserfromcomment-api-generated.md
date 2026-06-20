## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nej |  |
| banEmailDomain | boolean | query | Nej |  |
| banIP | boolean | query | Nej |  |
| deleteAllUsersComments | boolean | query | Nej |  |
| bannedUntil | string | query | Nej |  |
| isShadowBan | boolean | query | Nej |  |
| updateId | string | query | Nej |  |
| banReason | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (valgfri)
	banEmailDomain := true // bool |  (valgfri)
	banIP := true // bool |  (valgfri)
	deleteAllUsersComments := true // bool |  (valgfri)
	bannedUntil := "bannedUntil_example" // string |  (valgfri)
	isShadowBan := true // bool |  (valgfri)
	updateId := "updateId_example" // string |  (valgfri)
	banReason := "banReason_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]