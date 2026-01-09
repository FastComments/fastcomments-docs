Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| parentTenantId | string | query | לא |  |
| includeStats | boolean | query | לא |  |

## תגובה

מחזיר: [`AggregationResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregation_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-Aggregate'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	parentTenantId := "parentTenantId_example" // string |  (אופציונלי)
	includeStats := true // bool |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.Aggregate(context.Background()).TenantId(tenantId).AggregationRequest(aggregationRequest).ParentTenantId(parentTenantId).IncludeStats(includeStats).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.Aggregate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`Aggregate`: AggregationResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.Aggregate`: %v\n", resp)
}
[inline-code-end]

---