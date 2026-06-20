## Параметры

| Имя | Тип | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Нет |  |
| banEmailDomain | boolean | query | Нет |  |
| banIP | boolean | query | Нет |  |
| deleteAllUsersComments | boolean | query | Нет |  |
| bannedUntil | string | query | Нет |  |
| isShadowBan | boolean | query | Нет |  |
| updateId | string | query | Нет |  |
| banReason | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Пример

[inline-code-attrs-start title = 'Пример PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (необязательно)
	banEmailDomain := true // bool |  (необязательно)
	banIP := true // bool |  (необязательно)
	deleteAllUsersComments := true // bool |  (необязательно)
	bannedUntil := "bannedUntil_example" // string |  (необязательно)
	isShadowBan := true // bool |  (необязательно)
	updateId := "updateId_example" // string |  (необязательно)
	banReason := "banReason_example" // string |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]