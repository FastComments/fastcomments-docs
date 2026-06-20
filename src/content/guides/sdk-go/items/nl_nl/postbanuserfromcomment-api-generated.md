---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nee |  |
| banEmailDomain | boolean | query | Nee |  |
| banIP | boolean | query | Nee |  |
| deleteAllUsersComments | boolean | query | Nee |  |
| bannedUntil | string | query | Nee |  |
| isShadowBan | boolean | query | Nee |  |
| updateId | string | query | Nee |  |
| banReason | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (optioneel)
	banEmailDomain := true // bool |  (optioneel)
	banIP := true // bool |  (optioneel)
	deleteAllUsersComments := true // bool |  (optioneel)
	bannedUntil := "bannedUntil_example" // string |  (optioneel)
	isShadowBan := true // bool |  (optioneel)
	updateId := "updateId_example" // string |  (optioneel)
	banReason := "banReason_example" // string |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]

---