## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |

## Ответ

Возвращает: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_subscriptions_api_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetSubscriptions'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	userId := "userId_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSubscriptions(context.Background()).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSubscriptions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetSubscriptions`: GetSubscriptionsAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSubscriptions`: %v\n", resp)
}
[inline-code-end]

---