## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| forceRecalculate | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_aggregate_question_results_response.go)

## उदाहरण

[inline-code-attrs-start title = 'BulkAggregateQuestionResults उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	forceRecalculate := true // bool |  (वैकल्पिक)

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