## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_page_api_response.go)

## उदाहरण

[inline-code-attrs-start title = 'AddPage उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createAPIPageData := *openapiclient.NewCreateAPIPageData("Title_example", "Url_example", "UrlId_example") // CreateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddPage(context.Background()).TenantId(tenantId).CreateAPIPageData(createAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `AddPage` से प्रतिक्रिया: AddPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddPage`: %v\n", resp)
}
[inline-code-end]