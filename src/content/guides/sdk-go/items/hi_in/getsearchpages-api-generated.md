## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| value | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_page_search_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetSearchPages उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	value := "value_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchPages(context.Background()).TenantId(tenantId).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchPages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSearchPages` से प्रतिक्रिया: ModerationPageSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchPages`: %v\n", resp)
}
[inline-code-end]