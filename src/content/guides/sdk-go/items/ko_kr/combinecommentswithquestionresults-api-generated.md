## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니오 |  |
| questionIds | array | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| startDate | string | query | 아니오 |  |
| forceRecalculate | boolean | query | 아니오 |  |
| minValue | number | query | 아니오 |  |
| maxValue | number | query | 아니오 |  |
| limit | number | query | 아니오 |  |

## 응답

반환: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## 예제

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// `CombineCommentsWithQuestionResults`의 응답: CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]

---