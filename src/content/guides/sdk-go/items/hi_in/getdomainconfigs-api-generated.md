## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_configs_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetDomainConfigs उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetDomainConfigs(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`DefaultAPI.GetDomainConfigs` को कॉल करते समय त्रुटि: %v\n", err)
		fmt.Fprintf(os.Stderr, "पूर्ण HTTP प्रतिक्रिया: %v\n", r)
	}
	// response from `GetDomainConfigs`: GetDomainConfigsResponse
	fmt.Fprintf(os.Stdout, "`DefaultAPI.GetDomainConfigs` से प्रतिक्रिया: %v\n", resp)
}
[inline-code-end]