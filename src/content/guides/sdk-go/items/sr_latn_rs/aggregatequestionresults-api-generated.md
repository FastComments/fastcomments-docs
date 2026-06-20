## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| timeBucket | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |

## Odgovor

Vraća: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_response.go)

## Primer

[inline-code-attrs-start title = 'Primer AggregateQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (opciono)
	questionIds := []string{"Inner_example"} // []string |  (opciono)
	urlId := "urlId_example" // string |  (opciono)
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (opciono)
	startDate := time.Now() // time.Time |  (opciono)
	forceRecalculate := true // bool |  (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AggregateQuestionResults`: AggregateQuestionResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]