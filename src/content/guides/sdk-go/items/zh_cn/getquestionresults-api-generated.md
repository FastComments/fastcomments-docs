## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetQuestionResults 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string |  （可选）
	userId := "userId_example" // string |  （可选）
	startDate := "startDate_example" // string |  （可选）
	questionId := "questionId_example" // string |  （可选）
	questionIds := "questionIds_example" // string |  （可选）
	skip := float64(1.2) // float64 |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetQuestionResults` 的响应: GetQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]

---