## Параметры

| Name | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Нет |  |
| questionIds | array | query | Нет |  |
| urlId | string | query | Нет |  |
| timeBucket | string | query | Нет |  |
| startDate | string | query | Нет |  |
| forceRecalculate | boolean | query | Нет |  |

## Ответ

Возвращает: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример AggregateQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (необязательно)
	startDate := time.Now() // time.Time |  (необязательно)
	forceRecalculate := true // bool |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `AggregateQuestionResults`: AggregateQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]