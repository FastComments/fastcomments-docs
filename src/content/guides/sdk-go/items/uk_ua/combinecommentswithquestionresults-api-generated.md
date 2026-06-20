## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| questionId | string | query | Ні |  |
| questionIds | array | query | Ні |  |
| urlId | string | query | Ні |  |
| startDate | string | query | Ні |  |
| forceRecalculate | boolean | query | Ні |  |
| minValue | number | query | Ні |  |
| maxValue | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	questionId := "questionId_example" // string |  (необов'язково)
	questionIds := []string{"Inner_example"} // []string |  (необов'язково)
	urlId := "urlId_example" // string |  (необов'язково)
	startDate := time.Now() // time.Time |  (необов'язково)
	forceRecalculate := true // bool |  (необов'язково)
	minValue := float64(1.2) // float64 |  (необов'язково)
	maxValue := float64(1.2) // float64 |  (необов'язково)
	limit := float64(1.2) // float64 |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CombineCommentsWithQuestionResults`: CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]