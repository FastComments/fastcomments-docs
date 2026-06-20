## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| questionId | string | query | Όχι |  |
| questionIds | array | query | Όχι |  |
| urlId | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| forceRecalculate | boolean | query | Όχι |  |
| minValue | number | query | Όχι |  |
| maxValue | number | query | Όχι |  |
| limit | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (προαιρετικό)
	questionIds := []string{"Inner_example"} // []string |  (προαιρετικό)
	urlId := "urlId_example" // string |  (προαιρετικό)
	startDate := time.Now() // time.Time |  (προαιρετικό)
	forceRecalculate := true // bool |  (προαιρετικό)
	minValue := float64(1.2) // float64 |  (προαιρετικό)
	maxValue := float64(1.2) // float64 |  (προαιρετικό)
	limit := float64(1.2) // float64 |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `CombineCommentsWithQuestionResults`: CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]