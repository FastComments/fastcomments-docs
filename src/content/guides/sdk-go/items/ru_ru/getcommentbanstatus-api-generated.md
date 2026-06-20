## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_ban_status_response.go)

## Пример

[inline-code-attrs-start title = 'Пример использования GetCommentBanStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.ModerationAPI.GetCommentBanStatus(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCommentBanStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetCommentBanStatus`: GetCommentBanStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCommentBanStatus`: %v\n", resp)
}
[inline-code-end]

---