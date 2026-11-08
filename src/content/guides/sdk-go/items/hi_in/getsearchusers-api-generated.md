## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| value | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_user_search_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetSearchUsers उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	value := "value_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchUsers(context.Background()).TenantId(tenantId).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSearchUsers`: ModerationUserSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchUsers`: %v\n", resp)
}
[inline-code-end]