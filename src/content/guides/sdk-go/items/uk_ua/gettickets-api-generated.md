---
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| state | number | query | Ні |  |
| skip | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetTickets'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (необов'язково)
	state := float64(1.2) // float64 |  (необов'язково)
	skip := float64(1.2) // float64 |  (необов'язково)
	limit := float64(1.2) // float64 |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]

---