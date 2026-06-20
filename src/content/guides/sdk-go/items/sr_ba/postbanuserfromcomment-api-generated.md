## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| banEmail | boolean | query | Ne |  |
| banEmailDomain | boolean | query | Ne |  |
| banIP | boolean | query | Ne |  |
| deleteAllUsersComments | boolean | query | Ne |  |
| bannedUntil | string | query | Ne |  |
| isShadowBan | boolean | query | Ne |  |
| updateId | string | query | Ne |  |
| banReason | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Primjer

[inline-code-attrs-start title = 'Primjer PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (opciono)
	banEmailDomain := true // bool |  (opciono)
	banIP := true // bool |  (opciono)
	deleteAllUsersComments := true // bool |  (opciono)
	bannedUntil := "bannedUntil_example" // string |  (opciono)
	isShadowBan := true // bool |  (opciono)
	updateId := "updateId_example" // string |  (opciono)
	banReason := "banReason_example" // string |  (opciono)
	sso := "sso_example" // string |  (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]