## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| forceRecalculate | boolean | query | いいえ |  |

## レスポンス

戻り値: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_aggregate_question_results_response.go)

## 例

[inline-code-attrs-start title = 'BulkAggregateQuestionResults の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	bulkAggregateQuestionResultsRequest := *openapiclient.NewBulkAggregateQuestionResultsRequest([]openapiclient.BulkAggregateQuestionItem{*openapiclient.NewBulkAggregateQuestionItem("AggId_example")}) // BulkAggregateQuestionResultsRequest | 
	forceRecalculate := true // bool |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.BulkAggregateQuestionResults(context.Background()).TenantId(tenantId).BulkAggregateQuestionResultsRequest(bulkAggregateQuestionResultsRequest).ForceRecalculate(forceRecalculate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.BulkAggregateQuestionResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `BulkAggregateQuestionResults`: BulkAggregateQuestionResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.BulkAggregateQuestionResults`: %v\n", resp)
}
[inline-code-end]