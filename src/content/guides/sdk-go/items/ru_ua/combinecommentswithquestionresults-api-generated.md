## Параметры

| Name | Type | Location | Обязательно | Description |
|------|------|----------|------------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Нет |  |
| questionIds | array | query | Нет |  |
| urlId | string | query | Нет |  |
| startDate | string | query | Нет |  |
| forceRecalculate | boolean | query | Нет |  |
| minValue | number | query | Нет |  |
| maxValue | number | query | Нет |  |
| limit | number | query | Нет |  |

## Ответ

Возвращает: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (необязательно)
	questionIds := []string{"Inner_example"} // []string |  (необязательно)
	urlId := "urlId_example" // string |  (необязательно)
	startDate := time.Now() // time.Time |  (необязательно)
	forceRecalculate := true // bool |  (необязательно)
	minValue := float64(1.2) // float64 |  (необязательно)
	maxValue := float64(1.2) // float64 |  (необязательно)
	limit := float64(1.2) // float64 |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]