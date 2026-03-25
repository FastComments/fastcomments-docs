## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_subscription_api_response.go)

## Пример

[inline-code-attrs-start title = 'Пример UpdateSubscription'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateAPIUserSubscriptionData := *openapiclient.NewUpdateAPIUserSubscriptionData() // UpdateAPIUserSubscriptionData | 
	userId := "userId_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateSubscription(context.Background(), id).TenantId(tenantId).UpdateAPIUserSubscriptionData(updateAPIUserSubscriptionData).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateSubscription`: UpdateSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateSubscription`: %v\n", resp)
}
[inline-code-end]

---