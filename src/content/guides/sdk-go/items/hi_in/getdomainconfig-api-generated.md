## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domain | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetDomainConfig उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	domain := "domain_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetDomainConfig(context.Background(), domain).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDomainConfig`: GetDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetDomainConfig`: %v\n", resp)
}
[inline-code-end]