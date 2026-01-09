## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| questionId | string | query | Nie |  |
| questionIds | array | query | Nie |  |
| urlId | string | query | Nie |  |
| startDate | string | query | Nie |  |
| forceRecalculate | boolean | query | Nie |  |
| minValue | number | query | Nie |  |
| maxValue | number | query | Nie |  |
| limit | number | query | Nie |  |

## Odpowiedź

Zwraca: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	questionId := "questionId_example" // string |  (opcjonalne)
	questionIds := []string{"Inner_example"} // []string |  (opcjonalne)
	urlId := "urlId_example" // string |  (opcjonalne)
	startDate := time.Now() // time.Time |  (opcjonalne)
	forceRecalculate := true // bool |  (opcjonalne)
	minValue := float64(1.2) // float64 |  (opcjonalne)
	maxValue := float64(1.2) // float64 |  (opcjonalne)
	limit := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]