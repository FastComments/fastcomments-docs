## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |
| minValue | number | query | Hayır |  |
| maxValue | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## Örnek

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (isteğe bağlı)
	questionIds := []string{"Inner_example"} // []string |  (isteğe bağlı)
	urlId := "urlId_example" // string |  (isteğe bağlı)
	startDate := time.Now() // time.Time |  (isteğe bağlı)
	forceRecalculate := true // bool |  (isteğe bağlı)
	minValue := float64(1.2) // float64 |  (isteğe bağlı)
	maxValue := float64(1.2) // float64 |  (isteğe bağlı)
	limit := float64(1.2) // float64 |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CombineCommentsWithQuestionResults`'ten dönen yanıt: CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]

---