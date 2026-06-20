## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | шлях | Так |  |
| banEmail | boolean | параметр запиту | Ні |  |
| banEmailDomain | boolean | параметр запиту | Ні |  |
| banIP | boolean | параметр запиту | Ні |  |
| deleteAllUsersComments | boolean | параметр запиту | Ні |  |
| bannedUntil | string | параметр запиту | Ні |  |
| isShadowBan | boolean | параметр запиту | Ні |  |
| updateId | string | параметр запиту | Ні |  |
| banReason | string | параметр запиту | Ні |  |
| sso | string | параметр запиту | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Приклад

[inline-code-attrs-start title = 'Приклад PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	banEmail := true // bool |  (необов'язково)
	banEmailDomain := true // bool |  (необов'язково)
	banIP := true // bool |  (необов'язково)
	deleteAllUsersComments := true // bool |  (необов'язково)
	bannedUntil := "bannedUntil_example" // string |  (необов'язково)
	isShadowBan := true // bool |  (необов'язково)
	updateId := "updateId_example" // string |  (необов'язково)
	banReason := "banReason_example" // string |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `PostBanUserFromComment`: BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserFromComment`: %v\n", resp)
}
[inline-code-end]