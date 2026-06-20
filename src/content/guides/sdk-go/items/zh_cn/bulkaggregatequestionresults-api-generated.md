## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| forceRecalculate | boolean | query | 否 |  |

## 响应

返回: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_aggregate_question_results_response.go)

## 示例

[inline-code-attrs-start title = 'BulkAggregateQuestionResults 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	forceRecalculate := true // bool |  (可选)

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

---