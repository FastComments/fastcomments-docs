## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| tag | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_hash_tag_response.go)

## उदाहरण

[inline-code-attrs-start title = 'PatchHashTag उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	tag := "tag_example" // string | 
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PatchHashTag` से प्रतिक्रिया: UpdateHashTagResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]