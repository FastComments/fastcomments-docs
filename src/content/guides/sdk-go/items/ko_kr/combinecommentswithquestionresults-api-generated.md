## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니요 |  |
| questionIds | array | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| startDate | string | query | 아니요 |  |
| forceRecalculate | boolean | query | 아니요 |  |
| minValue | number | query | 아니요 |  |
| maxValue | number | query | 아니요 |  |
| limit | number | query | 아니요 |  |

## 응답

반환: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## 예제

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (선택 사항)
	questionIds := []string{"Inner_example"} // []string |  (선택 사항)
	urlId := "urlId_example" // string |  (선택 사항)
	startDate := time.Now() // time.Time |  (선택 사항)
	forceRecalculate := true // bool |  (선택 사항)
	minValue := float64(1.2) // float64 |  (선택 사항)
	maxValue := float64(1.2) // float64 |  (선택 사항)
	limit := float64(1.2) // float64 |  (선택 사항)

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

---