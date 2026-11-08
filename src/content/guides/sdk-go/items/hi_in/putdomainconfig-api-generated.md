## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domainToUpdate | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_put_domain_config_response.go)

## उदाहरण

[inline-code-attrs-start title = 'PutDomainConfig उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	domainToUpdate := "domainToUpdate_example" // string | 
	updateDomainConfigParams := *openapiclient.NewUpdateDomainConfigParams("Domain_example") // UpdateDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PutDomainConfig(context.Background(), domainToUpdate).TenantId(tenantId).UpdateDomainConfigParams(updateDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PutDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PutDomainConfig` से प्रतिक्रिया: PutDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutDomainConfig`: %v\n", resp)
}
[inline-code-end]