## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_package_response.go)

## उदाहरण

[inline-code-attrs-start title = 'CreateTenantPackage उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createTenantPackageBody := *openapiclient.NewCreateTenantPackageBody("Name_example", float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), false, "ForWhoText_example", []string{"FeatureTaglines_example"}, false) // CreateTenantPackageBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenantPackage(context.Background()).TenantId(tenantId).CreateTenantPackageBody(createTenantPackageBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenantPackage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateTenantPackage` से प्रतिक्रिया: CreateTenantPackageResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenantPackage`: %v\n", resp)
}
[inline-code-end]