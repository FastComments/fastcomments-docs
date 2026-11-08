## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comment_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetComment उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComment(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetComment` से प्रतिक्रिया: APIGetCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComment`: %v\n", resp)
}
[inline-code-end]