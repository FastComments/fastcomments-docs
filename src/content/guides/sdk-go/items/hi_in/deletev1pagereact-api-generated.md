## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## उदाहरण

[inline-code-attrs-start title = 'DeleteV1PageReact उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteV1PageReact(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteV1PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteV1PageReact` से प्रतिक्रिया: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteV1PageReact`: %v\n", resp)
}
[inline-code-end]