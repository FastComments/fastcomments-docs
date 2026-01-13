## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |

## Ответ

Возвращает: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_votes_for_user_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetVotesForUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 
	userId := "userId_example" // string |  (необязательно)
	anonUserId := "anonUserId_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetVotesForUser(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetVotesForUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetVotesForUser`: GetVotesForUser200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetVotesForUser`: %v\n", resp)
}
[inline-code-end]