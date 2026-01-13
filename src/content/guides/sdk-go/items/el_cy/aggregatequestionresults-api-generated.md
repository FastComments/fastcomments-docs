## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| questionId | string | query | Όχι |  |
| questionIds | array | query | Όχι |  |
| urlId | string | query | Όχι |  |
| timeBucket | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| forceRecalculate | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα AggregateQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (προαιρετικό)
	questionIds := []string{"Inner_example"} // []string |  (προαιρετικό)
	urlId := "urlId_example" // string |  (προαιρετικό)
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (προαιρετικό)
	startDate := time.Now() // time.Time |  (προαιρετικό)
	forceRecalculate := true // bool |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `AggregateQuestionResults`: AggregateQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]