## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetQuestionResults 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (可選)
	userId := "userId_example" // string |  (可選)
	startDate := "startDate_example" // string |  (可選)
	questionId := "questionId_example" // string |  (可選)
	questionIds := "questionIds_example" // string |  (可選)
	skip := float64(1.2) // float64 |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetQuestionResults` 的回應: GetQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]

---