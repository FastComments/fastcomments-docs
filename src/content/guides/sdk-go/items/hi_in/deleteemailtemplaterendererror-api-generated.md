## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| errorId | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## उदाहरण

[inline-code-attrs-start title = 'DeleteEmailTemplateRenderError उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	errorId := "errorId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteEmailTemplateRenderError(context.Background(), id, errorId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteEmailTemplateRenderError``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteEmailTemplateRenderError`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteEmailTemplateRenderError`: %v\n", resp)
}
[inline-code-end]