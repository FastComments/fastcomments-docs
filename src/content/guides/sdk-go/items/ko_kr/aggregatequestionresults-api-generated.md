## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니오 |  |
| questionIds | array | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| timeBucket | string | query | 아니오 |  |
| startDate | string | query | 아니오 |  |
| forceRecalculate | boolean | query | 아니오 |  |

## 응답

반환: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_question_results_200_response.go)

## 예제

[inline-code-attrs-start title = 'AggregateQuestionResults 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	timeBucket := openapiclient.AggregateTimeBucket("day") // AggregateTimeBucket |  (선택 사항)
	startDate := time.Now() // time.Time |  (선택 사항)
	forceRecalculate := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AggregateQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).TimeBucket(timeBucket).StartDate(startDate).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `AggregateQuestionResults`의 응답: AggregateQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]