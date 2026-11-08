groupBy दिए जाने पर दस्तावेज़ों को समूहित करके (grouping) और अनेक ऑपरेशन लागू करके उन्हें एकत्रित (aggregate) करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| parentTenantId | string | query | नहीं |  |
| includeStats | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`AggregateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_response.go)

## उदाहरण

[inline-code-attrs-start title = 'Aggregate उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	aggregationRequest := *openapiclient.NewAggregationRequest("ResourceName_example", []openapiclient.AggregationOperation{*openapiclient.NewAggregationOperation("Field_example", openapiclient.AggregationOpType("sum"))}) // AggregationRequest | 
	parentTenantId := "parentTenantId_example" // string |  (optional)
	includeStats := true // bool |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.Aggregate(context.Background()).TenantId(tenantId).AggregationRequest(aggregationRequest).ParentTenantId(parentTenantId).IncludeStats(includeStats).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.Aggregate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `Aggregate` से प्रतिक्रिया: AggregateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.Aggregate`: %v\n", resp)
}
[inline-code-end]