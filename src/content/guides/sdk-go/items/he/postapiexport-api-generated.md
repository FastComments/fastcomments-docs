## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| sso | string | query | לא |  |

## Response

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Example

[inline-code-attrs-start title = 'דוגמת PostApiExport'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (אופציונלי)
	byIPFromComment := "byIPFromComment_example" // string |  (אופציונלי)
	filters := "filters_example" // string |  (אופציונלי)
	searchFilters := "searchFilters_example" // string |  (אופציונלי)
	sorts := "sorts_example" // string |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "שגיאה בעת קריאה ל-`ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "תגובה מלאה של HTTP: %v\n", r)
	}
	// תגובה מ-`PostApiExport`: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "תגובה מ-`ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]