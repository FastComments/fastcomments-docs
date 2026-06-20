## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_from_comment_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetBanUsersFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetBanUsersFromComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetBanUsersFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetBanUsersFromComment`: GetBannedUsersFromCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetBanUsersFromComment`: %v\n", resp)
}
[inline-code-end]