## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_result_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetQuestionResult'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetQuestionResult(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResult``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetQuestionResult`: GetQuestionResult200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResult`: %v\n", resp)
}
[inline-code-end]

---