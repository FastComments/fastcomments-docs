## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postId | string | path | हाँ |  |
| isUndo | boolean | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_react_feed_post_response.go)

## उदाहरण

[inline-code-attrs-start title = 'ReactFeedPostPublic उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postId := "postId_example" // string | 
	reactBodyParams := *openapiclient.NewReactBodyParams() // ReactBodyParams | 
	isUndo := true // bool |  (वैकल्पिक)
	broadcastId := "broadcastId_example" // string |  (वैकल्पिक)
	sso := "sso_example" // string |  (वैकल्पिक)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ReactFeedPostPublic(context.Background(), tenantId, postId).ReactBodyParams(reactBodyParams).IsUndo(isUndo).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ReactFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `ReactFeedPostPublic` से प्रतिक्रिया: ReactFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ReactFeedPostPublic`: %v\n", resp)
}
[inline-code-end]