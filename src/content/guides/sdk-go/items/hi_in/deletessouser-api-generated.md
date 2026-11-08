## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| deleteComments | boolean | query | नहीं |  |
| commentDeleteMode | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_sso_user_api_response.go)

## उदाहरण

[inline-code-attrs-start title = 'DeleteSSOUser उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	deleteComments := true // bool |  (वैकल्पिक)
	commentDeleteMode := "commentDeleteMode_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSSOUser(context.Background(), id).TenantId(tenantId).DeleteComments(deleteComments).CommentDeleteMode(commentDeleteMode).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteSSOUser` से प्रतिक्रिया: DeleteSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSSOUser`: %v\n", resp)
}
[inline-code-end]