## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentIds | string | query | हाँ | कमेंट आईडी की कॉमा से अलग की गई सूची। |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_check_blocked_comments_response.go)

## उदाहरण

[inline-code-attrs-start title = 'CheckedCommentsForBlocked उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentIds := "commentIds_example" // string | कमेंट आईडी की कॉमा से अलग की गई सूची।
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CheckedCommentsForBlocked(context.Background()).TenantId(tenantId).CommentIds(commentIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CheckedCommentsForBlocked``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CheckedCommentsForBlocked` से प्रतिक्रिया: CheckBlockedCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CheckedCommentsForBlocked`: %v\n", resp)
}
[inline-code-end]