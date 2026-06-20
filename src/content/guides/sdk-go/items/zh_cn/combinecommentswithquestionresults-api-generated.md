## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | Yes |  |
| questionId | string | 查询 | No |  |
| questionIds | array | 查询 | No |  |
| urlId | string | 查询 | No |  |
| startDate | string | 查询 | No |  |
| forceRecalculate | boolean | 查询 | No |  |
| minValue | number | 查询 | No |  |
| maxValue | number | 查询 | No |  |
| limit | number | 查询 | No |  |

## 响应

返回: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_question_results_with_comments_response.go)

## 示例

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  （可选）
	questionIds := []string{"Inner_example"} // []string |  （可选）
	urlId := "urlId_example" // string |  （可选）
	startDate := time.Now() // time.Time |  （可选）
	forceRecalculate := true // bool |  （可选）
	minValue := float64(1.2) // float64 |  （可选）
	maxValue := float64(1.2) // float64 |  （可选）
	limit := float64(1.2) // float64 |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `CombineCommentsWithQuestionResults` 的响应：CombineQuestionResultsWithCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]