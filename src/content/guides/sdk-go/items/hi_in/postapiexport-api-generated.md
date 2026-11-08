## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filters | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| sorts | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## उदाहरण

[inline-code-attrs-start title = 'PostApiExport उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (वैकल्पिक)
	byIPFromComment := "byIPFromComment_example" // string |  (वैकल्पिक)
	filters := "filters_example" // string |  (वैकल्पिक)
	searchFilters := "searchFilters_example" // string |  (वैकल्पिक)
	sorts := "sorts_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostApiExport` से प्रतिक्रिया: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]