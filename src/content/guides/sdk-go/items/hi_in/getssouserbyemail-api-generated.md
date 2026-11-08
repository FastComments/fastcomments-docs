## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| email | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_user_by_email_api_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetSSOUserByEmail उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	email := "email_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUserByEmail(context.Background(), email).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUserByEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSSOUserByEmail` से प्रतिक्रिया: GetSSOUserByEmailAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUserByEmail`: %v\n", resp)
}
[inline-code-end]