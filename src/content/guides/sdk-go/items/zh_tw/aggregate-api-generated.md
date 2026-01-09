透過將文件分組（如果提供了 groupBy）並套用多個操作來彙總文件。
支援不同的操作（例如 sum、countDistinct、avg 等）。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| parentTenantId | string | query | 否 |  |
| includeStats | boolean | query | 否 |  |

## 回應

回傳: [`AggregationResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregation_response.go)

## 範例

[inline-code-attrs-start title = '彙總範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	aggregationRequest := *openapiclient.NewAggregationRequest("ResourceName_example", []openapiclient.AggregationOperation{*openapiclient.NewAggregationOperation("Field_example", openapiclient.AggregationOpType("sum"))}) // AggregationRequest | 
	parentTenantId := "parentTenantId_example" // string |  (可選)
	includeStats := true // bool |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.Aggregate(context.Background()).TenantId(tenantId).AggregationRequest(aggregationRequest).ParentTenantId(parentTenantId).IncludeStats(includeStats).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.Aggregate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `Aggregate` 的回應: AggregationResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.Aggregate`: %v\n", resp)
}
[inline-code-end]