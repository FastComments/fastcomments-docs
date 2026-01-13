## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| questionId | string | query | לא |  |
| questionIds | array | query | לא |  |
| urlId | string | query | לא |  |
| startDate | string | query | לא |  |
| forceRecalculate | boolean | query | לא |  |
| minValue | number | query | לא |  |
| maxValue | number | query | לא |  |
| limit | number | query | לא |  |

## תגובה

מחזיר: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת CombineCommentsWithQuestionResults'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (אופציונלי)
	questionIds := []string{"Inner_example"} // []string |  (אופציונלי)
	urlId := "urlId_example" // string |  (אופציונלי)
	startDate := time.Now() // time.Time |  (אופציונלי)
	forceRecalculate := true // bool |  (אופציונלי)
	minValue := float64(1.2) // float64 |  (אופציונלי)
	maxValue := float64(1.2) // float64 |  (אופציונלי)
	limit := float64(1.2) // float64 |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תשובה מ-`CombineCommentsWithQuestionResults`: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]