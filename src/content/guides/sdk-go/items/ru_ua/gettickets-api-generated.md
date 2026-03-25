## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| state | number | query | Нет |  |
| skip | number | query | Нет |  |
| limit | number | query | Нет |  |

## Ответ

Возвращает: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetTickets'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (необязательно)
	state := float64(1.2) // float64 |  (необязательно)
	skip := float64(1.2) // float64 |  (необязательно)
	limit := float64(1.2) // float64 |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]