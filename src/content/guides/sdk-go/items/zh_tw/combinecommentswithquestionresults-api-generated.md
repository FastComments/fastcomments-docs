## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |
| minValue | number | query | 否 |  |
| maxValue | number | query | 否 |  |
| limit | number | query | 否 |  |

## 回應

回傳: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## 範例

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  （可選）
	questionIds := []string{"Inner_example"} // []string |  （可選）
	urlId := "urlId_example" // string |  （可選）
	startDate := time.Now() // time.Time |  （可選）
	forceRecalculate := true // bool |  （可選）
	minValue := float64(1.2) // float64 |  （可選）
	maxValue := float64(1.2) // float64 |  （可選）
	limit := float64(1.2) // float64 |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CombineCommentsWithQuestionResults` 的回應： CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]