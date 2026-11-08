## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_question_configs_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetQuestionConfigs उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetQuestionConfigs(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetQuestionConfigs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetQuestionConfigs`: GetQuestionConfigsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetQuestionConfigs`: %v\n", resp)
}
[inline-code-end]