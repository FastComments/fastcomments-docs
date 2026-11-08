## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| trustFactor | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_user_trust_factor_response.go)

## उदाहरण

[inline-code-attrs-start title = 'SetTrustFactor उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (वैकल्पिक)
	trustFactor := "trustFactor_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.SetTrustFactor(context.Background()).TenantId(tenantId).UserId(userId).TrustFactor(trustFactor).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.SetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SetTrustFactor` से प्रतिक्रिया: SetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.SetTrustFactor`: %v\n", resp)
}
[inline-code-end]