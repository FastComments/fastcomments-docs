Fasst Dokumente zusammen, indem sie gruppiert werden (falls groupBy angegeben ist) und mehrere Operationen angewendet werden.
Verschiedene Operationen (z. B. sum, countDistinct, avg usw.) werden unterstützt.

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Antwort

Gibt zurück: [`AggregationResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregation_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für Aggregation'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	parentTenantId := "parentTenantId_example" // string |  (optional)
	includeStats := true // bool |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.Aggregate(context.Background()).TenantId(tenantId).AggregationRequest(aggregationRequest).ParentTenantId(parentTenantId).IncludeStats(includeStats).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.Aggregate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `Aggregate`: AggregationResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.Aggregate`: %v\n", resp)
}
[inline-code-end]