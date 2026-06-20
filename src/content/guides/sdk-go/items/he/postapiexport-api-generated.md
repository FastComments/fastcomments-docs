## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-PostApiExport'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string | (אופציונלי)
	byIPFromComment := "byIPFromComment_example" // string | (אופציונלי)
	filters := "filters_example" // string | (אופציונלי)
	searchFilters := "searchFilters_example" // string | (אופציונלי)
	sorts := "sorts_example" // string | (אופציונלי)
	sso := "sso_example" // string | (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`PostApiExport`: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]