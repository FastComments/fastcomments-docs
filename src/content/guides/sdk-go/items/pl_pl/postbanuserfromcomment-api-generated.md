## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| banEmail | boolean | query | Nie |  |
| banEmailDomain | boolean | query | Nie |  |
| banIP | boolean | query | Nie |  |
| deleteAllUsersComments | boolean | query | Nie |  |
| bannedUntil | string | query | Nie |  |
| isShadowBan | boolean | query | Nie |  |
| updateId | string | query | Nie |  |
| banReason | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (opcjonalne)
	banEmailDomain := true // bool |  (opcjonalne)
	banIP := true // bool |  (opcjonalne)
	deleteAllUsersComments := true // bool |  (opcjonalne)
	bannedUntil := "bannedUntil_example" // string |  (opcjonalne)
	isShadowBan := true // bool |  (opcjonalne)
	updateId := "updateId_example" // string |  (opcjonalne)
	banReason := "banReason_example" // string |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]