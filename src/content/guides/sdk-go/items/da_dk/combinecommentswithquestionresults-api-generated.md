## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nej |  |
| questionIds | array | query | Nej |  |
| urlId | string | query | Nej |  |
| startDate | string | query | Nej |  |
| forceRecalculate | boolean | query | Nej |  |
| minValue | number | query | Nej |  |
| maxValue | number | query | Nej |  |
| limit | number | query | Nej |  |

## Svar

Returnerer: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (valgfri)
	questionIds := []string{"Inner_example"} // []string |  (valgfri)
	urlId := "urlId_example" // string |  (valgfri)
	startDate := time.Now() // time.Time |  (valgfri)
	forceRecalculate := true // bool |  (valgfri)
	minValue := float64(1.2) // float64 |  (valgfri)
	maxValue := float64(1.2) // float64 |  (valgfri)
	limit := float64(1.2) // float64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]