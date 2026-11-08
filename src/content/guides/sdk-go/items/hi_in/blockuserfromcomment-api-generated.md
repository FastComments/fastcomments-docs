## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BlockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_block_success.go)

## उदाहरण

[inline-code-attrs-start title = 'BlockUserFromComment उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	blockFromCommentParams := *openapiclient.NewBlockFromCommentParams() // BlockFromCommentParams | 
	userId := "userId_example" // string |  (वैकल्पिक)
	anonUserId := "anonUserId_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.BlockUserFromComment(context.Background(), id).TenantId(tenantId).BlockFromCommentParams(blockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.BlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `BlockUserFromComment` से प्रतिक्रिया: BlockSuccess
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.BlockUserFromComment`: %v\n", resp)
}
[inline-code-end]