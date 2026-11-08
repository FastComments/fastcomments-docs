## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| meta | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenants_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetTenants उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	meta := "meta_example" // string |  (वैकल्पिक)
	skip := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenants(context.Background()).TenantId(tenantId).Meta(meta).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenants` से प्रतिक्रिया: GetTenantsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenants`: %v\n", resp)
}
[inline-code-end]