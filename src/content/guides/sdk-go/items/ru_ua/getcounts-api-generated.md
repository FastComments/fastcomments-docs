## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_count_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetCounts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	sso := "sso_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCounts(context.Background()).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetCounts`: GetBannedUsersCountResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCounts`: %v\n", resp)
}
[inline-code-end]