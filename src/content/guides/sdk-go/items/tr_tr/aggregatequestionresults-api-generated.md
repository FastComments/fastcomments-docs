## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| timeBucket | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |

## Yanıt

Dönen değer: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_200_response.go)

## Örnek

[inline-code-attrs-start title = 'AggregateQuestionResults Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (isteğe bağlı)
	questionIds := []string{"Inner_example"} // []string |  (isteğe bağlı)
	urlId := "urlId_example" // string |  (isteğe bağlı)
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (isteğe bağlı)
	startDate := time.Now() // time.Time |  (isteğe bağlı)
	forceRecalculate := true // bool |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Çağrı yapılırken `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// yanıt `AggregateQuestionResults`'ten: AggregateQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Yanıt `DefaultAPI.AggregateQuestionResults`'ten: %v\n", resp)
}
[inline-code-end]