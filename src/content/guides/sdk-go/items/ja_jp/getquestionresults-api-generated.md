## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| questionId | string | query | いいえ |  |
| questionIds | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_results_response.go)

## 例

[inline-code-attrs-start title = 'GetQuestionResults の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string |  (任意)
	userId := "userId_example" // string |  (任意)
	startDate := "startDate_example" // string |  (任意)
	questionId := "questionId_example" // string |  (任意)
	questionIds := "questionIds_example" // string |  (任意)
	skip := float64(1.2) // float64 |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionResults(context.Background()).TenantId(tenantId).UrlId(urlId).UserId(userId).StartDate(startDate).QuestionId(questionId).QuestionIds(questionIds).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetQuestionResults` のレスポンス: GetQuestionResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionResults`: %v\n", resp)
}
[inline-code-end]

---