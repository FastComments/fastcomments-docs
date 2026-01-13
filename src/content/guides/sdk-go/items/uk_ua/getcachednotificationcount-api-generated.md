## Параметри

| Назва | Type | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_cached_notification_count_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetCachedNotificationCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetCachedNotificationCount(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetCachedNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetCachedNotificationCount`: GetCachedNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetCachedNotificationCount`: %v\n", resp)
}
[inline-code-end]