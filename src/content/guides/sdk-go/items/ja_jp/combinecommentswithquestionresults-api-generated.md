## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| questionId | string | query | いいえ |  |
| questionIds | array | query | いいえ |  |
| urlId | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| forceRecalculate | boolean | query | いいえ |  |
| minValue | number | query | いいえ |  |
| maxValue | number | query | いいえ |  |
| limit | number | query | いいえ |  |

## レスポンス

戻り値: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_combine_comments_with_question_results_200_response.go)

## 例

[inline-code-attrs-start title = 'CombineCommentsWithQuestionResults の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	questionId := "questionId_example" // string |  (任意)
	questionIds := []string{"Inner_example"} // []string |  (任意)
	urlId := "urlId_example" // string |  (任意)
	startDate := time.Now() // time.Time |  (任意)
	forceRecalculate := true // bool |  (任意)
	minValue := float64(1.2) // float64 |  (任意)
	maxValue := float64(1.2) // float64 |  (任意)
	limit := float64(1.2) // float64 |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CombineCommentsWithQuestionResults(context.Background()).TenantId(tenantId).QuestionId(questionId).QuestionIds(questionIds).UrlId(urlId).StartDate(startDate).ForceRecalculate(forceRecalculate).MinValue(minValue).MaxValue(maxValue).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CombineCommentsWithQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CombineCommentsWithQuestionResults` のレスポンス: CombineCommentsWithQuestionResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CombineCommentsWithQuestionResults`: %v\n", resp)
}
[inline-code-end]