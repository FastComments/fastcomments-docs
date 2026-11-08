## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_create_hash_tags_response.go)

## उदाहरण

[inline-code-attrs-start title = 'AddHashTagsBulk उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	bulkCreateHashTagsBody := *openapiclient.NewBulkCreateHashTagsBody([]openapiclient.BulkCreateHashTagsBodyTagsInner{*openapiclient.NewBulkCreateHashTagsBodyTagsInner("Tag_example")}) // BulkCreateHashTagsBody |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTagsBulk(context.Background()).TenantId(tenantId).BulkCreateHashTagsBody(bulkCreateHashTagsBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTagsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AddHashTagsBulk`: BulkCreateHashTagsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTagsBulk`: %v\n", resp)
}
[inline-code-end]