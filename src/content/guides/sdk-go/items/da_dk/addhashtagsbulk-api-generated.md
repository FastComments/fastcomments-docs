## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_create_hash_tags_response.go)

## Eksempel

[inline-code-attrs-start title = 'AddHashTagsBulk Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string |  (valgfri)
	bulkCreateHashTagsBody := *openapiclient.NewBulkCreateHashTagsBody([]openapiclient.BulkCreateHashTagsBodyTagsInner{*openapiclient.NewBulkCreateHashTagsBodyTagsInner("Tag_example")}) // BulkCreateHashTagsBody |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTagsBulk(context.Background()).TenantId(tenantId).BulkCreateHashTagsBody(bulkCreateHashTagsBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTagsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `AddHashTagsBulk`: BulkCreateHashTagsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTagsBulk`: %v\n", resp)
}
[inline-code-end]