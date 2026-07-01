## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Example

[inline-code-attrs-start title = 'PostApiExport 例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (optional) → (オプション)
	byIPFromComment := "byIPFromComment_example" // string |  (optional) → (オプション)
	filters := "filters_example" // string |  (optional) → (オプション)
	searchFilters := "searchFilters_example" // string |  (optional) → (オプション)
	sorts := "sorts_example" // string |  (optional) → (オプション)
	sso := "sso_example" // string |  (optional) → (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err) // 呼び出し時のエラー `ModerationAPI.PostApiExport`: %v
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r) // 完全な HTTP レスポンス: %v
	}
	// response from `PostApiExport`: ModerationExportResponse → `PostApiExport` からの応答: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp) // `ModerationAPI.PostApiExport` からの応答: %v
}
[inline-code-end]