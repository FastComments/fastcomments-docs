## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_user_response.go)

## उदाहरण

[inline-code-attrs-start title = 'CreateTenantUser उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createTenantUserBody := *openapiclient.NewCreateTenantUserBody("Username_example", "Email_example") // CreateTenantUserBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenantUser(context.Background()).TenantId(tenantId).CreateTenantUserBody(createTenantUserBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateTenantUser` से प्रतिक्रिया: CreateTenantUserResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenantUser`: %v\n", resp)
}
[inline-code-end]