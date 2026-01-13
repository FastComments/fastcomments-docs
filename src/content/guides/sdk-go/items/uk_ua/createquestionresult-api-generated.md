## Параметри

| Назва | Type | Location | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Повертає

Повертає: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_question_result_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад CreateQuestionResult'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createQuestionResultBody := *openapiclient.NewCreateQuestionResultBody("UrlId_example", float64(123), "QuestionId_example") // CreateQuestionResultBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateQuestionResult(context.Background()).TenantId(tenantId).CreateQuestionResultBody(createQuestionResultBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateQuestionResult``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `CreateQuestionResult`: CreateQuestionResult200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateQuestionResult`: %v\n", resp)
}
[inline-code-end]