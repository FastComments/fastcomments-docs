## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| banEmail | boolean | query | Όχι |  |
| banEmailDomain | boolean | query | Όχι |  |
| banIP | boolean | query | Όχι |  |
| deleteAllUsersComments | boolean | query | Όχι |  |
| bannedUntil | string | query | Όχι |  |
| isShadowBan | boolean | query | Όχι |  |
| updateId | string | query | Όχι |  |
| banReason | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (προαιρετικό)
	banEmailDomain := true // bool |  (προαιρετικό)
	banIP := true // bool |  (προαιρετικό)
	deleteAllUsersComments := true // bool |  (προαιρετικό)
	bannedUntil := "bannedUntil_example" // string |  (προαιρετικό)
	isShadowBan := true // bool |  (προαιρετικό)
	updateId := "updateId_example" // string |  (προαιρετικό)
	banReason := "banReason_example" // string |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]

---