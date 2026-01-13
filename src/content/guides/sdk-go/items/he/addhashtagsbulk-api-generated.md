## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## תגובה

מחזיר: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_hash_tags_bulk_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-AddHashTagsBulk'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string |  (אופציונלי)
	bulkCreateHashTagsBody := *openapiclient.NewBulkCreateHashTagsBody([]openapiclient.BulkCreateHashTagsBodyTagsInner{*openapiclient.NewBulkCreateHashTagsBodyTagsInner("Tag_example")}) // BulkCreateHashTagsBody |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddHashTagsBulk(context.Background()).TenantId(tenantId).BulkCreateHashTagsBody(bulkCreateHashTagsBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddHashTagsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`AddHashTagsBulk`: AddHashTagsBulk200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddHashTagsBulk`: %v\n", resp)
}
[inline-code-end]