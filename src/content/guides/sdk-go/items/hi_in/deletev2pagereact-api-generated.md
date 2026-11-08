## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| id | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_v1_page_react.go)

## उदाहरण

[inline-code-attrs-start title = 'DeleteV2PageReact उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteV2PageReact(context.Background(), tenantId).UrlId(urlId).Id(id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteV2PageReact``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteV2PageReact`: CreateV1PageReact
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteV2PageReact`: %v\n", resp)
}
[inline-code-end]