---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nee |  |
| questionIds | array | query | Nee |  |
| urlId | string | query | Nee |  |
| startDate | string | query | Nee |  |
| forceRecalculate | boolean | query | Nee |  |
| minValue | number | query | Nee |  |
| maxValue | number | query | Nee |  |
| limit | number | query | Nee |  |

## Antwoord

Geeft terug: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (optioneel)
	questionIds := []string{"Inner_example"} // []string |  (optioneel)
	urlId := "urlId_example" // string |  (optioneel)
	startDate := time.Now() // time.Time |  (optioneel)
	forceRecalculate := true // bool |  (optioneel)
	minValue := float64(1.2) // float64 |  (optioneel)
	maxValue := float64(1.2) // float64 |  (optioneel)
	limit := float64(1.2) // float64 |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `CombineCommentsWithQuestionResults`: CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]

---