## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| spam | boolean | query | नहीं |  |
| permNotSpam | boolean | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## उदाहरण

[inline-code-attrs-start title = 'PostSetCommentSpamStatus उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	spam := true // bool |  (वैकल्पिक)
	permNotSpam := true // bool |  (वैकल्पिक)
	broadcastId := "broadcastId_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentSpamStatus(context.Background(), commentId).TenantId(tenantId).Spam(spam).PermNotSpam(permNotSpam).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentSpamStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PostSetCommentSpamStatus` से प्रतिक्रिया: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentSpamStatus`: %v\n", resp)
}
[inline-code-end]