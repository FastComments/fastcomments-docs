## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Пример

[inline-code-attrs-start title = 'Пример за PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (по избор)
	banEmailDomain := true // bool |  (по избор)
	banIP := true // bool |  (по избор)
	deleteAllUsersComments := true // bool |  (по избор)
	bannedUntil := "bannedUntil_example" // string |  (по избор)
	isShadowBan := true // bool |  (по избор)
	updateId := "updateId_example" // string |  (по избор)
	banReason := "banReason_example" // string |  (по избор)
	sso := "sso_example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]