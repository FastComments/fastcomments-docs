## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_moderator_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetModerator'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetModerator(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetModerator``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetModerator`: GetModerator200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetModerator`: %v\n", resp)
}
[inline-code-end]

---