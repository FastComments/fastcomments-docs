## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| startDate | string | query | Nie |  |
| questionId | string | query | Nie |  |
| questionIds | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (opcjonalne)
	userId := "userId_example" // string |  (opcjonalne)
	startDate := "startDate_example" // string |  (opcjonalne)
	questionId := "questionId_example" // string |  (opcjonalne)
	questionIds := "questionIds_example" // string |  (opcjonalne)
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetQuestionResults`: GetQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]