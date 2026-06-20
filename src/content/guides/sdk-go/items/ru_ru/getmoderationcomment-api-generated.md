## Параметры

| Имя | Тип | Местоположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| includeEmail | boolean | query | Нет |  |
| includeIP | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_comment_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetModerationComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeEmail := true // bool |  (необязательно)
	includeIP := true // bool |  (необязательно)
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationComment(context.Background(), commentId).IncludeEmail(includeEmail).IncludeIP(includeIP).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetModerationComment`: ModerationAPICommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationComment`: %v\n", resp)
}
[inline-code-end]