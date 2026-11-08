## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| skip | integer | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_users_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetSSOUsers उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := int32(56) // int32 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSSOUsers`: GetSSOUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUsers`: %v\n", resp)
}
[inline-code-end]