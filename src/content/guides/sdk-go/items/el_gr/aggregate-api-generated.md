Συγκεντρώνει έγγραφα ομαδοποιώντας τα (εφόσον παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| parentTenantId | string | query | Όχι |  |
| includeStats | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_aggregate_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα Aggregate'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	parentTenantId := "parentTenantId_example" // string |  (προαιρετικό)
	includeStats := true // bool |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.Aggregate(context.Background()).TenantId(tenantId).AggregationRequest(aggregationRequest).ParentTenantId(parentTenantId).IncludeStats(includeStats).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.Aggregate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `Aggregate`: AggregateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.Aggregate`: %v\n", resp)
}
[inline-code-end]