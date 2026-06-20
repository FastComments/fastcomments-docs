## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_pre_ban_summary.go)

## Example

[inline-code-attrs-start title = 'PostBulkPreBanSummary Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	bulkPreBanParams := *openapiclient.NewBulkPreBanParams([]string{"CommentIds_example"}) // BulkPreBanParams | 
	includeByUserIdAndEmail := true // bool |  (optional)
	includeByIP := true // bool |  (optional)
	includeByEmailDomain := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBulkPreBanSummary(context.Background()).BulkPreBanParams(bulkPreBanParams).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBulkPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostBulkPreBanSummary`: BulkPreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBulkPreBanSummary`: %v\n", resp)
}
[inline-code-end]
